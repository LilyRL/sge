use bon::Builder;
use glium::{
    PolygonMode,
    glutin::{
        config::{ColorBufferType, ConfigSurfaceTypes, ConfigTemplateBuilder},
        context::{ContextAttributesBuilder, GlProfile, Priority, ReleaseBehavior, Robustness},
        surface::{SurfaceAttributesBuilder, SwapInterval, WindowSurface},
    },
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter},
    winit::{
        dpi::{Position, Size},
        window::{Cursor, Fullscreen, Icon, Theme, WindowAttributes, WindowButtons, WindowLevel},
    },
};
use log::{LevelFilter, info};
use sge_camera::get_cameras;
use sge_global::global;
use sge_types::Verbosity;
use sge_window::WindowOptions;

#[derive(Builder)]
pub struct Opts {
    /// The type of the backing buffer and ancillary buffers.
    pub template_color_buffer_type: Option<ColorBufferType>,

    /// Bits of alpha in the color buffer.
    pub template_alpha_size: Option<u8>,

    /// Bits of depth in the depth buffer.
    pub template_depth_size: Option<u8>,

    /// Bits of stencil in the stencil buffer.
    pub template_stencil_size: Option<u8>,

    /// The amount of samples in multisample buffer.
    pub template_num_samples: Option<u8>,

    /// The types of the surfaces supported by the configuration.
    pub template_config_surface_types: Option<ConfigSurfaceTypes>,

    /// The config should support transparency.
    pub template_transparency: Option<bool>,

    /// The config should prefer single buffering.
    pub template_single_buffering: Option<bool>,

    /// The config supports stereoscopy.
    pub template_stereoscopy: Option<bool>,

    /// The config uses floating pixels.
    pub template_float_pixels: Option<bool>,

    /// The config should prefer hardware accelerated formats.
    pub template_hardware_accelerated: Option<bool>,

    /// The maximum height of the pbuffer.
    pub template_max_pbuffer_height: Option<u32>,

    /// The same as in
    /// [`SurfaceAttributesBuilder::<WindowSurface>::with_single_buffer`].
    pub surface_single_buffer: Option<bool>,

    /// Sets the *debug* flag for the OpenGL context.
    ///
    /// Debug contexts are usually slower, but give better error reporting.
    /// This option is ignored when using [`Robustness::NoError`].
    ///
    /// The default value for this flag is `false`.
    pub opengl_debug: Option<bool>,

    /// Sets the robustness of the OpenGL context. See the docs of
    /// [`Robustness`].
    ///
    /// The default is [`Robustness::NotRobust`], because this is what typically
    /// expected when you create an OpenGL context.  However for safety you
    /// should consider [`Robustness::RobustLoseContextOnReset`].
    pub opengl_robustness: Option<Robustness>,

    /// The behavior when changing the current context. See the docs of
    /// [`ReleaseBehavior`].
    ///
    /// The default is [`ReleaseBehavior::Flush`].
    pub opengl_release_behaviour: Option<ReleaseBehavior>,

    /// Set the desired OpenGL context profile. See the docs of [`GlProfile`].
    ///
    /// By default the profile is unspecified.
    ///
    /// # Api-specific
    ///
    /// - **macOS:** not supported, the latest is picked automatically.
    pub opengl_profile: Option<GlProfile>,

    /// Set the priority hint, which might not be honored if the API does not
    /// support it, if there are constraints on the number of high priority
    /// contexts available in the system, or system policy limits access to
    /// high priority contexts to appropriate system privilege level the
    /// context creation may fail.
    ///
    /// By default no priority is specified, which corresponds to
    /// [`Priority::Medium`].
    ///
    /// # Api specific
    ///
    /// - **WGL/GLX:** not implemented.
    /// - **CGL:** not supported.
    pub opengl_priority: Option<Priority>,

    // window_attributes stuff
    pub window_inner_size: Option<Size>,
    pub window_min_inner_size: Option<Size>,
    pub window_max_inner_size: Option<Size>,
    pub window_position: Option<Position>,
    pub window_resizable: Option<bool>,
    pub window_enabled_buttons: Option<WindowButtons>,
    pub window_maximized: Option<bool>,
    pub window_visible: Option<bool>,
    pub window_transparent: Option<bool>,
    pub window_blur: Option<bool>,
    pub window_decorations: Option<bool>,
    pub window_window_icon: Option<Icon>,
    pub window_preferred_theme: Option<Theme>,
    pub window_resize_increments: Option<Size>,
    pub window_content_protected: Option<bool>,
    pub window_window_level: Option<WindowLevel>,
    pub window_active: Option<bool>,
    pub window_cursor: Option<Cursor>,
    pub window_fullscreen: Option<Fullscreen>,

    pub swap_interval: Option<SwapInterval>,
    pub min_log_level: Option<LevelFilter>,
    pub log_verbosity: Option<Verbosity>,
    pub use_mipmaps: Option<bool>,
    pub default_magnify_filter: Option<MagnifySamplerFilter>,
    pub default_minify_filter: Option<MinifySamplerFilter>,
    pub wait_for_events: Option<bool>,
    pub title: String,
    pub dithering: Option<bool>,
    pub polygon_mode: Option<PolygonMode>,
    pub line_width: Option<f32>,
}

impl Opts {
    pub fn build(self) -> EngineCreationOptions {
        let default = EngineCreationOptions::default();

        let template = {
            let mut default = ConfigTemplateBuilder::default();

            if let Some(color_buffer_type) = self.template_color_buffer_type {
                default = default.with_buffer_type(color_buffer_type);
            }

            if let Some(alpha_size) = self.template_alpha_size {
                default = default.with_alpha_size(alpha_size);
            }

            if let Some(depth_size) = self.template_depth_size {
                default = default.with_depth_size(depth_size);
            }

            if let Some(stencil_size) = self.template_stencil_size {
                default = default.with_stencil_size(stencil_size);
            }

            if let Some(num_samples) = self.template_num_samples {
                default = default.with_multisampling(num_samples);
            }

            if let Some(config_surface_types) = self.template_config_surface_types {
                default = default.with_surface_type(config_surface_types);
            }

            if let Some(transparency) = self.template_transparency {
                default = default.with_transparency(transparency);
            }

            if let Some(single_buffering) = self.template_single_buffering {
                default = default.with_single_buffering(single_buffering);
            }

            if let Some(stereoscopy) = self.template_stereoscopy {
                default = default.with_stereoscopy(Some(stereoscopy));
            }

            if let Some(float_pixels) = self.template_float_pixels {
                default = default.with_float_pixels(float_pixels);
            }

            if let Some(hardware_accelerated) = self.template_hardware_accelerated {
                default = default.prefer_hardware_accelerated(Some(hardware_accelerated));
            }

            default
        };

        let surface_attributes = {
            let mut default = SurfaceAttributesBuilder::<WindowSurface>::default();

            if let Some(single_buffer) = self.surface_single_buffer {
                default = default.with_single_buffer(single_buffer);
            }

            default
        };

        let context_attributes = {
            let mut default = ContextAttributesBuilder::default();

            if let Some(debug) = self.opengl_debug {
                default = default.with_debug(debug);
            }

            if let Some(robustness) = self.opengl_robustness {
                default = default.with_robustness(robustness);
            }

            if let Some(release_behaviour) = self.opengl_release_behaviour {
                default = default.with_release_behavior(release_behaviour);
            }

            if let Some(profile) = self.opengl_profile {
                default = default.with_profile(profile);
            }

            if let Some(priority) = self.opengl_priority {
                default = default.with_priority(priority);
            }

            default
        };

        let swap_interval = self.swap_interval.unwrap_or(default.window.swap_interval);

        let window_attributes = {
            let mut default = WindowAttributes::default();

            if let Some(inner_size) = self.window_inner_size {
                default.inner_size = Some(inner_size);
            }

            if let Some(min_inner_size) = self.window_min_inner_size {
                default.min_inner_size = Some(min_inner_size);
            }

            if let Some(max_inner_size) = self.window_max_inner_size {
                default.max_inner_size = Some(max_inner_size);
            }

            if let Some(position) = self.window_position {
                default.position = Some(position);
            }

            if let Some(resizable) = self.window_resizable {
                default.resizable = resizable;
            }

            if let Some(enabled_buttons) = self.window_enabled_buttons {
                default.enabled_buttons = enabled_buttons;
            }

            if let Some(maximized) = self.window_maximized {
                default.maximized = maximized;
            }

            if let Some(visible) = self.window_visible {
                default.visible = visible;
            }

            if let Some(transparent) = self.window_transparent {
                default.transparent = transparent;
            } else {
                default.transparent = true;
            }

            if let Some(blur) = self.window_blur {
                default.blur = blur;
            }

            if let Some(decorations) = self.window_decorations {
                default.decorations = decorations;
            }

            if let Some(window_icon) = self.window_window_icon {
                default.window_icon = Some(window_icon);
            }

            if let Some(preferred_theme) = self.window_preferred_theme {
                default.preferred_theme = Some(preferred_theme);
            }

            if let Some(resize_increments) = self.window_resize_increments {
                default.resize_increments = Some(resize_increments);
            }

            if let Some(content_protected) = self.window_content_protected {
                default.content_protected = content_protected;
            }

            if let Some(window_level) = self.window_window_level {
                default.window_level = window_level;
            }

            if let Some(active) = self.window_active {
                default.active = active;
            }

            if let Some(cursor) = self.window_cursor {
                default.cursor = cursor;
            }

            if let Some(fullscreen) = self.window_fullscreen {
                default.fullscreen = Some(fullscreen);
            }

            default.title = self.title;

            default
        };

        let window = WindowOptions {
            template,
            surface_attributes,
            context_attributes,
            swap_interval,
            window_attributes,
        };

        let min_log_level = self.min_log_level.unwrap_or(default.min_log_level);
        let log_verbosity = self.log_verbosity.unwrap_or(default.log_verbosity);
        let use_mipmaps = self.use_mipmaps.unwrap_or(default.config.use_mipmaps);
        let default_magnify_filter = self
            .default_magnify_filter
            .unwrap_or(default.config.default_magnify_filter);
        let default_minify_filter = self
            .default_minify_filter
            .unwrap_or(default.config.default_minify_filter);
        let wait_for_events = self
            .wait_for_events
            .unwrap_or(default.config.wait_for_events);
        let dithering = self.dithering.unwrap_or(default.config.dithering);
        let polygon_mode = self.polygon_mode.unwrap_or(default.config.polygon_mode);

        let config = EngineConfig {
            use_mipmaps,
            default_magnify_filter,
            default_minify_filter,
            wait_for_events,
            dithering,
            use_positive_y_up: false,
            polygon_mode,
            line_width: self.line_width,
        };

        EngineCreationOptions {
            window,
            min_log_level,
            log_verbosity,
            config,
        }
    }
}

pub struct EngineCreationOptions {
    pub window: WindowOptions,
    pub min_log_level: LevelFilter,
    pub log_verbosity: Verbosity,
    pub config: EngineConfig,
}

impl EngineCreationOptions {
    pub fn builder() -> OptsBuilder {
        Opts::builder()
    }
}

impl Default for EngineCreationOptions {
    fn default() -> Self {
        Self {
            window: WindowOptions::default(),
            min_log_level: LevelFilter::Info,
            config: EngineConfig::default(),
            log_verbosity: Verbosity::Medium,
        }
    }
}

pub struct EngineConfig {
    /// applies when loading a texture, not drawing
    ///
    /// setting this to true will make textures look better (less horrible and pixelated) from afer
    ///
    /// setting this to false will sometimes make images look crisper
    pub use_mipmaps: bool,
    pub default_magnify_filter: MagnifySamplerFilter,
    pub default_minify_filter: MinifySamplerFilter,
    /// makes the engine wait for new events before re-rendering
    pub wait_for_events: bool,
    pub dithering: bool,
    pub use_positive_y_up: bool,
    pub polygon_mode: PolygonMode,
    pub line_width: Option<f32>,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            use_mipmaps: true,
            default_magnify_filter: MagnifySamplerFilter::Nearest,
            default_minify_filter: MinifySamplerFilter::LinearMipmapLinear,
            wait_for_events: false,
            dithering: true,
            use_positive_y_up: false,
            polygon_mode: PolygonMode::Fill,
            line_width: None,
        }
    }
}

global!(EngineConfig, config);

pub fn init(config: EngineConfig) {
    set_config(config);
    info!("Initialized sge_config");
}

// applies when loading a texture, not drawing
//
// setting this to true will make textures look better (less horrible and pixelated) from afer
//
// setting this to false will sometimes make images look crisper
pub fn use_mipmaps(use_mipmaps: bool) {
    get_config().use_mipmaps = use_mipmaps;
}

pub fn use_linear_filtering() {
    get_config().default_magnify_filter = MagnifySamplerFilter::Linear;
    get_config().default_minify_filter = MinifySamplerFilter::Linear;
}

pub fn use_default_filtering() {
    get_config().default_magnify_filter = MagnifySamplerFilter::Linear;
    get_config().default_minify_filter = MinifySamplerFilter::LinearMipmapLinear;
}

pub fn use_nearest_filtering() {
    get_config().default_magnify_filter = MagnifySamplerFilter::Nearest;
    get_config().default_minify_filter = MinifySamplerFilter::Nearest;
}

pub fn set_minify_filter(filtering: MinifySamplerFilter) {
    get_config().default_minify_filter = filtering;
}

pub fn set_magnify_filter(filtering: MagnifySamplerFilter) {
    get_config().default_magnify_filter = filtering;
}

pub fn wait_for_events() {
    get_config().wait_for_events = true;
}

pub fn dont_wait_for_events() {
    get_config().wait_for_events = false;
}

pub fn set_wait_for_events(wait_for_events: bool) {
    get_config().wait_for_events = wait_for_events;
}

pub fn get_wait_for_events() -> bool {
    get_config().wait_for_events
}

pub fn get_wait_for_events_mut() -> &'static mut bool {
    &mut get_config().wait_for_events
}

pub fn toggle_wait_for_events() {
    let config = get_config();
    config.wait_for_events = !config.wait_for_events;
}

pub fn use_dithering() {
    get_config().dithering = true;
}

pub fn dont_use_dithering() {
    get_config().dithering = false;
}

pub fn get_dithering_mut() -> &'static mut bool {
    &mut get_config().dithering
}

pub fn get_dithering() -> bool {
    get_config().dithering
}

/// this is flipped from the default
pub fn use_positive_y_up() {
    get_config().use_positive_y_up = true;
    get_cameras().set_flip_y(true);
}

/// this is the default
pub fn use_positive_y_down() {
    get_config().use_positive_y_up = false;
    get_cameras().set_flip_y(false);
}

pub fn set_polygon_mode(mode: PolygonMode) {
    get_config().polygon_mode = mode;
}

pub fn get_polygon_mode() -> PolygonMode {
    get_config().polygon_mode
}

pub fn toggle_wireframe() {
    match get_polygon_mode() {
        PolygonMode::Fill => set_polygon_mode(PolygonMode::Line),
        PolygonMode::Line => set_polygon_mode(PolygonMode::Fill),
        _ => (),
    }
}
