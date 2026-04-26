use sge_window::window_center;

use super::*;

#[derive(Debug)]
pub struct Modal {
    child: Child,
    overlay: Child,
    open: *mut bool,
}

impl Modal {
    pub fn new(child: Child, overlay: Child, open: &mut bool) -> UiRef {
        Self {
            child,
            overlay,
            open: open as *mut bool,
        }
        .to_ref()
    }

    fn open(&self) -> bool {
        unsafe { *self.open }
    }

    pub(crate) fn actually_draw(&self, ui: &UiState) {
        let size = window_size();
        let overlay_area = Area::new(Vec2::ZERO, size);
        self.overlay.draw(overlay_area, ui);

        let size = self.child.preferred_dimensions();
        let top_left = window_center() - size / 2.0;
        let child_area = Area::new(top_left, size);
        self.child.draw(child_area, ui);
    }
}

impl UiNode for Modal {
    fn draw(&self, _: Area, _: &UiState) -> Vec2 {
        if !self.open() {
            return Vec2::ZERO;
        }

        get_ui_storage().modals.push(self as *const Self);

        Vec2::ZERO
    }

    fn preferred_dimensions(&self) -> Vec2 {
        Vec2::ZERO
    }

    fn size(&self, _: Area) -> Vec2 {
        Vec2::ZERO
    }
}
