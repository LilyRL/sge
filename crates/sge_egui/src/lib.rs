pub use egui_glium;

use egui_glium::{
    EguiGlium,
    egui_winit::egui::{Context, ViewportId},
};
use sge_window::{get_display, get_window_state};

sge_global::sge_global!(EguiState, egui_state);

pub struct EguiState {
    pub egui: EguiGlium,
    pub initialized: bool,
}

pub fn init() {
    set_egui_state(EguiState {
        initialized: false,
        egui: EguiGlium::new(
            ViewportId::ROOT,
            get_display(),
            &get_window_state().window,
            &get_window_state().event_loop,
        ),
    });
    log::info!("Initialized egui");
}

pub fn egui() -> &'static mut EguiGlium {
    &mut get_egui_state().egui
}

/// call at end of frame
pub fn update() {
    get_egui_state().initialized = false;
}

pub fn run_egui(mut f: impl FnMut(&Context)) {
    let state = get_egui_state();
    state.initialized = true;
    state.egui.run(&get_window_state().window, |ctx| {
        f(ctx);
    });
}
