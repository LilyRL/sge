use crate::prelude::draw_rounded_rect;

use super::*;

pub struct Fill {
    fill_color: Color,
    corner_radius: f32,
    child: Child,
}

impl Fill {
    pub fn new(fill_color: Color, child: Child) -> UiRef {
        Self {
            fill_color,
            corner_radius: 0.0,
            child,
        }
        .to_ref()
    }

    pub fn rounded(fill_color: Color, corner_radius: f32, child: Child) -> UiRef {
        Self {
            fill_color,
            corner_radius,
            child,
        }
        .to_ref()
    }
}

impl UiNode for Fill {
    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        if self.corner_radius == 0.0 {
            area.fill(self.fill_color);
        } else {
            draw_rounded_rect(
                area.top_left,
                area.size,
                self.fill_color,
                self.corner_radius,
            );
        }

        self.child.node.draw(area, ui)
    }

    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }
}
