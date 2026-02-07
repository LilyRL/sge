use std::cell::{Cell, RefCell};

use log::warn;

use super::*;

pub struct Button {
    id: usize,
    child: Child,
}

impl Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Button")
            .field("child", &self.child)
            .finish()
    }
}

impl Button {
    pub fn new(id: usize, child: Child) -> UiRef {
        Self { id, child }.to_ref()
    }
}

impl UiNode for Button {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        if ui.is_hovered(area) && ui.input().mouse_released(MouseButton::Left) {
            get_state().storage.button_clicked = Some(self.id);
        }

        self.child.node.draw(area, ui)
    }
}

pub fn ui_button_clicked(id: usize) -> bool {
    get_state().storage.button_clicked == Some(id)
}
