use crate::prelude::draw_rounded_rect_with_outline;

use super::*;

pub struct RoundedFill {
    fill_color: Color,
    outline_color: Color,
    corner_radius: f32,
    outline_thickness: f32,
    child: Child,
}

impl RoundedFill {
    pub fn new(fill_color: Color, corner_radius: f32, child: Child) -> UiRef {
        Self {
            fill_color,
            outline_color: fill_color,
            corner_radius,
            outline_thickness: 0.0,
            child,
        }
        .to_ref()
    }

    pub fn with_outline(
        fill_color: Color,
        outline_color: Color,
        corner_radius: f32,
        outline_thickness: f32,
        child: Child,
    ) -> UiRef {
        Self {
            fill_color,
            outline_color,
            corner_radius,
            outline_thickness,
            child,
        }
        .to_ref()
    }
}

impl UiNode for RoundedFill {
    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        draw_rounded_rect_with_outline(
            area.top_left,
            area.size,
            self.fill_color,
            self.corner_radius,
            self.outline_thickness,
            self.outline_color,
        );

        self.child.node.draw(area, ui)
    }

    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }
}

pub struct RoundedHoverFill {
    base_color: Color,
    hovered_color: Color,
    corner_radius: f32,
    outline_thickness: f32,
    outline_color: Color,
    child: Child,
}

impl RoundedHoverFill {
    pub fn with_outline(
        base_color: Color,
        hovered_color: Color,
        corner_radius: f32,
        outline_thickness: f32,
        outline_color: Color,
        child: Child,
    ) -> UiRef {
        Self {
            base_color,
            hovered_color,
            corner_radius,
            outline_thickness,
            outline_color,
            child,
        }
        .to_ref()
    }

    pub fn new(base_color: Color, hovered_color: Color, corner_radius: f32, child: Child) -> UiRef {
        Self {
            base_color,
            hovered_color,
            corner_radius,
            outline_thickness: 0.0,
            outline_color: base_color,
            child,
        }
        .to_ref()
    }
}

impl UiNode for RoundedHoverFill {
    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let is_hovered = ui.input().is_cursor_within_area(area);
        let color = if is_hovered {
            self.hovered_color
        } else {
            self.base_color
        };

        draw_rounded_rect_with_outline(
            area.top_left,
            area.size,
            color,
            self.corner_radius,
            self.outline_thickness,
            self.outline_color,
        );

        self.child.node.draw(area, ui)
    }

    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }
}
