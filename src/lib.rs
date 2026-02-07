#![allow(static_mut_refs)]
#![feature(duration_millis_float)]
use std::collections::HashMap;
use std::time::Instant;

use bevy_math::Mat4;
use bevy_math::Vec2;
use camera::Camera2D;
use camera::Camera3D;
use camera::projection_from_window;
use color::Color;
use config::EngineConfig;
use config::EngineCreationOptions;
#[cfg(feature = "debugging")]
use debugging::DebugInfo;
pub use draw_queue_2d::Vertex3D;
use egui_glium::{EguiGlium, egui_winit::egui::ViewportId};
pub use engine_color as color;
use error_union::ErrorUnion;
use fps_ticker::Fps;
use glium::Program;
#[cfg(feature = "profile")]
use glium::glutin::surface::SwapInterval;
use glium::{
    Frame,
    backend::glutin::Display,
    glutin::surface::WindowSurface,
    winit::{
        event::Event, event_loop::EventLoop, platform::pump_events::EventLoopExtPumpEvents,
        window::Window,
    },
};
use image::Image;
use input::Input;
use log::error;
use log::info;
use logging::get_logger;
use materials::Material;
use object_3d::Mesh;
use object_3d::Object3D;
use prelude::TextureAtlas;
use prelude::init_fonts;
use prelude::init_materials;
use programs::init_programs;
use rand::rngs::ThreadRng;
use render_pipeline::RenderPipeline;
use render_pipeline::RenderTexture;
use text_rendering::EngineFont;
use textures::EngineTexture;
use textures::init_textures;
use tunes::engine::AudioEngine;
use ui::SomeNode;
use ui::SomeState;
use ui::StateRef;
use ui::update_ui;
use user_storage::UserStorage;
#[cfg(feature = "profile")]
use window::WindowOptions;
use window::init_window;

pub mod animation;
pub mod api;
pub mod camera;
pub mod collisions;
pub mod config;
#[cfg(feature = "debugging")]
mod debugging;
mod draw_queue_2d;
mod draw_queue_3d;
pub mod image;
mod input;
pub mod logging;
mod materials;
mod object_3d;
mod physics;
mod post_processing;
pub mod prelude;
mod programs;
mod render_pipeline;
pub mod scissor;
mod shapes_2d;
mod shapes_3d;
mod slop;
mod text_rendering;
mod textures;
pub mod transform;
pub mod ui;
mod user_storage;
mod utils;
pub mod window;

pub(crate) static mut ENGINE_STATE: Option<EngineState> = None;

fn get_state() -> &'static mut EngineState {
    // thread_assert::same_thread();

    unsafe { ENGINE_STATE.as_mut().unwrap_or_else(|| panic!()) }
}

type EngineDisplay = Display<WindowSurface>;

struct EngineState {
    window: Window,
    window_size: Vec2,

    display: EngineDisplay,
    event_loop: EventLoop<()>,
    // bump_allocator: Bump,
    input: Input,
    frame: Option<Frame>,
    /// used for screen-space rendering
    flat_projection: Mat4,
    camera_2d: Camera2D,
    camera_3d: Camera3D,
    egui: EguiGlium,
    audio_engine: AudioEngine,
    egui_initialized: bool,
    render_pipeline: RenderPipeline,
    texture_pipeline: Option<RenderPipeline>,
    #[cfg(feature = "profile")]
    profiler_guard: pprof::ProfilerGuard<'static>,
    #[cfg(feature = "profile")]
    fps: Fps,
    #[cfg(feature = "debugging")]
    debug_info: debugging::DebugInfo,
    storage: EngineStorage,
    rng: ThreadRng,
    config: EngineConfig,
    time: f32,
    physics_time: f32,
    physics_speed: f32,
    physics_delta_time: f32,
    is_physics_time_paused: bool,
    frame_count: usize,
    delta_time: f32,
    last_frame_end_time: Instant,
    cursor_position: Vec2,
    user_storage: UserStorage,
    scissors: Vec<glium::Rect>,
}

unsafe impl Sync for EngineState {}
unsafe impl Send for EngineState {}

pub(crate) struct EngineStorage {
    textures: Vec<EngineTexture>,
    render_textures: Vec<RenderTexture>,
    programs: Vec<Program>,
    materials: Vec<Material>,
    objects: Vec<Object3D>,
    fonts: Vec<EngineFont>,
    meshes: Vec<Mesh>,
    texture_atlasses: Vec<TextureAtlas>,
    images: Vec<Image>,
    ui_nodes: Vec<SomeNode>,
    ui_states: HashMap<StateRef, SomeState>,
    button_clicked: Option<usize>,
}

impl EngineStorage {
    pub fn new() -> Self {
        Self {
            textures: vec![],
            programs: vec![],
            materials: vec![],
            objects: vec![],
            render_textures: vec![],
            fonts: vec![],
            meshes: vec![],
            texture_atlasses: vec![],
            images: vec![],
            ui_nodes: vec![],
            ui_states: HashMap::new(),
            button_clicked: None,
        }
    }
}

pub fn init(title: impl AsRef<str>) -> Result<(), InitError> {
    let opts = EngineCreationOptions::builder()
        .title(title.as_ref().to_string())
        .build();
    init_custom(opts)
}

#[derive(ErrorUnion, Debug)]
pub enum InitError {
    SetLogger(log::SetLoggerError),
    WindowCreation(window::WindowCreationError),
    Program(glium::ProgramCreationError),
    Audio(tunes::error::TunesError),
}

#[allow(unused_mut)]
pub fn init_custom(mut opts: config::Opts) -> Result<(), InitError> {
    #[cfg(feature = "profile")]
    {
        opts.swap_interval = Some(SwapInterval::DontWait);
    }

    let opts = opts.build();
    logging::init_engine_logger()?;
    get_logger().min_log_level = opts.min_log_level;
    get_logger().verbosity = opts.log_verbosity;
    match color_eyre::install() {
        Ok(_) => (),
        Err(e) => error!("Could not install color_eyre: {e}"),
    }

    let (window, display, event_loop) = match init_window(opts.window) {
        Ok(output) => {
            info!("Initialized window, display, event loop.");
            output
        }
        Err(e) => {
            error!("FATAL: Could not initialize window: {e}");
            return Err(e.into());
        }
    };

    let input = Input::new();
    window.request_redraw();

    let frame = None;

    let flat_projection = projection_from_window(&window);
    let camera_2d = Camera2D::from_window(&window);
    let camera_3d = Camera3D::from_window(&window);
    let gui = EguiGlium::new(ViewportId::ROOT, &display, &window, &event_loop);
    #[cfg(feature = "debugging")]
    let debug_info = DebugInfo::new();
    let mut storage = EngineStorage::new();
    init_programs(&display, &mut storage)?;
    init_textures(&mut storage, &display);
    init_materials(&mut storage);
    let rng = rand::rng();
    let gui_initialized = false;
    let config = EngineConfig::default();
    let time = 0.0;
    let delta_time = 0.0;
    let last_frame_end_time = Instant::now();
    let render_pipeline = RenderPipeline::screen();
    let audio_engine = AudioEngine::new()?;
    let user_storage = UserStorage::new();

    let size = window.inner_size();
    let window_size = Vec2::new(size.width as f32, size.height as f32);
    // let bump_allocator = Bump::new();

    unsafe {
        ENGINE_STATE = Some(EngineState {
            window,
            window_size,
            display,
            texture_pipeline: None,
            event_loop,
            input,
            // bump_allocator,
            frame,
            flat_projection,
            camera_2d,
            camera_3d,
            audio_engine,
            egui: gui,
            egui_initialized: gui_initialized,
            #[cfg(feature = "debugging")]
            debug_info,
            storage,
            rng,
            render_pipeline,
            config,
            time,
            delta_time,
            last_frame_end_time,
            is_physics_time_paused: false,
            cursor_position: Vec2::ZERO,
            frame_count: 0,
            physics_time: 0.0,
            physics_delta_time: 0.0,
            physics_speed: 1.0,
            user_storage,
            #[cfg(feature = "profile")]
            fps: Fps::default(),
            #[cfg(feature = "profile")]
            profiler_guard: pprof::ProfilerGuardBuilder::default()
                .frequency(1000)
                .blocklist(&["libc", "libgcc", "pthread", "vdso"])
                .build()
                .unwrap(),
            scissors: vec![],
        });
    }

    thread_assert::set_thread_id();

    init_fonts();

    info!("Finished initializing engine.");

    Ok(())
}

pub fn next_frame() {
    update_ui();

    #[cfg(feature = "debugging")]
    let engine_start_time = Instant::now();
    let state = get_state();

    #[allow(deprecated)]
    state
        .event_loop
        .pump_events(None, |event, event_loop_window_target| match event {
            Event::WindowEvent { event, .. } => {
                let gui_response = state.egui.on_event(&state.window, &event);
                if gui_response.consumed {
                    return;
                }

                state.input.process_window_event(&event);

                if state.input.close_requested() {
                    event_loop_window_target.exit();
                }

                if let Some(size) = state.input.window_resized() {
                    state.display.resize(size.into());
                    state.flat_projection = projection_from_window(&state.window);
                    let size = state.window.inner_size();
                    state.camera_2d.update_sizes(size.width, size.height);
                    state.camera_3d.update_sizes(size.width, size.height);
                    state.window_size = Vec2::new(size.width as f32, size.height as f32);
                }
            }
            Event::DeviceEvent { event, .. } => {
                state.input.process_device_event(&event);
            }
            Event::NewEvents(_) => {
                state.input.step();
            }
            _ => (),
        });

    #[cfg(feature = "gamepad")]
    state.input.gamepad.update();

    let mut frame = state.frame.take().unwrap_or_else(|| state.display.draw());

    state.render_pipeline.draw_on(&mut frame);
    state.render_pipeline = RenderPipeline::screen();

    if state.egui_initialized {
        state.egui.paint(&state.display, &mut frame);
    }

    #[cfg(feature = "debugging")]
    {
        state.debug_info.next_frame();
        let engine_time = engine_start_time.elapsed();
        state.debug_info.current_frame_mut().engine_time = engine_time.as_millis_f64();
    }

    frame.finish().unwrap();
    state.window.request_redraw();

    state.frame = Some(state.display.draw());

    let delta_time = state.last_frame_end_time.elapsed().as_secs_f32();
    state.delta_time = delta_time;
    state.time += delta_time;
    state.last_frame_end_time = Instant::now();

    if !state.is_physics_time_paused {
        state.physics_delta_time = delta_time * state.physics_speed;
        state.physics_time += state.physics_delta_time;
    } else {
        state.physics_delta_time = 0.0;
    }

    state.frame_count += 1;

    if let Some(c) = state.input.cursor() {
        state.cursor_position = c.into();
    }

    #[cfg(feature = "profile")]
    {
        state.fps.tick();

        if state.time >= 1.0
            && let Ok(report) = state.profiler_guard.report().build()
        {
            use std::process::exit;

            let file = std::fs::File::create("flamegraph.svg").unwrap();
            report.flamegraph(file).unwrap();
            println!("FPS: {}", state.fps.avg());
            exit(0);
        };
    }
}

pub(crate) mod thread_assert {
    static mut THREAD_ID: Option<std::thread::ThreadId> = None;

    pub fn set_thread_id() {
        unsafe {
            THREAD_ID = Some(std::thread::current().id());
        }
    }

    #[allow(unused)]
    pub fn same_thread() {
        unsafe {
            thread_local! {
                static CURRENT_THREAD_ID: std::thread::ThreadId = std::thread::current().id();
            }
            assert!(THREAD_ID.is_some());
            assert!(THREAD_ID.unwrap() == CURRENT_THREAD_ID.with(|id| *id));
        }
    }
}

impl Drop for EngineState {
    fn drop(&mut self) {
        if let Some(frame) = self.frame.take() {
            let _ = frame.finish();
        }
    }
}

impl EngineState {
    pub(crate) fn window_size(&self) -> Vec2 {
        self.window_size
    }

    pub(crate) fn dpi_scaling(&self) -> f32 {
        self.window.scale_factor() as f32
    }
}
