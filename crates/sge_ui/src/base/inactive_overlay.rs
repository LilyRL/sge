use sge_api::area::AreaExt;

use super::*;

#[derive(Debug)]
pub struct InactiveOverlay {
    color: Color,
    child: Child,
}

impl InactiveOverlay {
    pub fn new(color: Color, child: Child) -> UiRef {
        Self { color, child }.to_ref()
    }
}

impl UiNode for InactiveOverlay {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.preferred_dimensions()
    }

    fn size(&self, area: Area) -> Vec2 {
        self.child.size(area)
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let size = self.child.draw(area, ui);

        if !ui.is_hovered(area) {
            area.fill(self.color);
        }

        size
    }
}
