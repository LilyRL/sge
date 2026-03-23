#![allow(static_mut_refs)]
#![allow(unused)]
#![feature(duration_millis_float)]
use std::collections::HashMap;
use std::time::Instant;

use bevy_math::Vec2;
use error_union::ErrorUnion;
use glium::winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::ActiveEventLoop,
    platform::pump_events::EventLoopExtPumpEvents,
};
use log::{error, info};
use sge_config::{EngineCreationOptions, Opts, get_config};
use sge_debugging::get_debug_info;
use sge_egui::{egui, get_egui_state};
use sge_input::get_input;
pub use sge_math::collision;
use sge_rendering::{get_render_state, pipeline::RenderPipeline};
pub use sge_shapes::d2 as shapes_2d;
use sge_time::frames_since_input;
use sge_window::{get_display, get_window_state, window_size, window_size_u32};

const WAIT_FOR_EVENTS_EXTRA_FRAME_DRAWS: usize = 60 * 5; // stops rendering after 300 frames of no input, when config.wait_for_events is true

pub mod api;
pub mod prelude;
mod user_storage;

pub fn init(title: impl AsRef<str>) -> Result<(), InitError> {
    let opts = Opts::builder().title(title.as_ref().to_string()).build();
    init_custom(opts)
}

#[derive(ErrorUnion, Debug)]
pub enum InitError {
    SetLogger(log::SetLoggerError),
    WindowCreation(sge_window::WindowCreationError),
    Program(glium::ProgramCreationError),
    Audio(sge_audio::AudioInitError),
    Gilrs(sge_input::GilrsError),
}

#[allow(unused_mut)]
pub fn init_custom(mut opts: Opts) -> Result<(), InitError> {
    let opts = opts.build();
    sge_logging::init()?;
    sge_logging::get_logger().min_log_level = opts.min_log_level;
    sge_logging::get_logger().verbosity = opts.log_verbosity;
    match color_eyre::install() {
        Ok(_) => (),
        Err(e) => error!("Could not install color_eyre: {e}"),
    }

    match sge_window::init(opts.window) {
        Ok(output) => {
            info!("Initialized window, display, event loop.");
            output
        }
        Err(e) => {
            error!("FATAL: Could not initialize window: {e}");
            return Err(e.into());
        }
    }

    let size = window_size_u32();

    sge_time::init();
    sge_config::init(opts.config);
    get_window_state().window.request_redraw();
    sge_input::init()?;
    sge_camera::init(size.width, size.height, false);
    sge_egui::init();
    sge_debugging::init();
    sge_rng::init();
    sge_programs::init()?;
    sge_rendering::init();
    sge_audio::init()?;
    sge_textures::init();
    sge_text::init();
    sge_ui::init_ui();
    user_storage::init();
    sge_physics::init();
    // sge_routines::init();

    info!("Finished initializing engine.");

    Ok(())
}

pub fn next_frame() {
    let engine_start_time = Instant::now();

    let has_input_event = process_events();

    sge_input::update();
    render_frame();
    sge_egui::update();
    sge_ui::update();
    // sge_routines::update();
    sge_time::update(has_input_event);
    record_frame_time(engine_start_time);
    request_redraw_if_needed();
}

fn process_events() -> bool {
    let mut has_input_event = false;
    let input = get_input();
    let state = get_window_state();

    #[allow(deprecated)]
    state
        .event_loop
        .pump_events(None, |event, event_loop_window_target| match event {
            Event::WindowEvent { event, .. } => {
                has_input_event |= handle_window_event(&event, event_loop_window_target);
            }
            Event::DeviceEvent { event, .. } => {
                input.process_device_event(&event);
                has_input_event = true;
            }
            Event::NewEvents(_) => {
                input.step();
            }
            _ => (),
        });

    has_input_event
}

fn handle_window_event(event: &WindowEvent, event_loop_window_target: &ActiveEventLoop) -> bool {
    let egui = egui();
    let input = get_input();
    let window = &get_window_state().window;
    let gui_response = egui.on_event(window, event);
    if gui_response.consumed {
        return false;
    }

    let is_input_event = !input.process_window_event(event);

    if input.close_requested() {
        event_loop_window_target.exit();
    }

    if let Some(size) = input.window_resized() {
        handle_window_resize(size);
    }

    is_input_event
}

fn handle_window_resize(size: PhysicalSize<u32>) {
    sge_window::handle_window_resize(size);
    sge_camera::update_cameras_on_resize(size.width, size.height);
}

fn render_frame() {
    let display = get_display();
    let pipeline = &mut get_render_state().render_pipeline;

    let mut frame = display.draw();
    pipeline.draw_on(&mut frame);
    *pipeline = RenderPipeline::screen();

    let state = get_egui_state();
    if state.initialized {
        state.egui.paint(get_display(), &mut frame);
    }

    frame.finish().unwrap();
}

fn request_redraw_if_needed() {
    let config = get_config();
    let window = get_window_state();
    if !config.wait_for_events || frames_since_input() < WAIT_FOR_EVENTS_EXTRA_FRAME_DRAWS {
        window.window.request_redraw();
    }
}

fn record_frame_time(engine_start_time: Instant) {
    let debug_info = get_debug_info();
    debug_info.next_frame();
    let engine_time = engine_start_time.elapsed();
    debug_info.current_frame_mut().engine_time = engine_time.as_millis_f64();
}
