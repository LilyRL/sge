use crate::prelude::draw_rounded_rect_with_outline;

use super::*;

pub struct FillStyle {
    pub fill: Color,
    pub outline: Color,
    pub corner_radius: f32,
    pub outline_thickness: f32,
}

pub struct ActiveFill {
    base: FillStyle,
    hover: FillStyle,
    active: FillStyle,
    child: Child,
}

impl ActiveFill {
    pub fn custom(base: FillStyle, hover: FillStyle, active: FillStyle, child: Child) -> UiRef {
        Self {
            base,
            hover,
            active,
            child,
        }
        .to_ref()
    }

    pub fn new(
        base_color: Color,
        hover_color: Color,
        active_color: Color,
        corner_radius: f32,
        child: Child,
    ) -> UiRef {
        Self {
            base: FillStyle {
                fill: base_color,
                outline: base_color,
                corner_radius,
                outline_thickness: 0.0,
            },
            hover: FillStyle {
                fill: hover_color,
                outline: hover_color,
                corner_radius,
                outline_thickness: 0.0,
            },
            active: FillStyle {
                fill: active_color,
                outline: active_color,
                corner_radius,
                outline_thickness: 0.0,
            },
            child,
        }
        .to_ref()
    }

    pub fn new_rounded(
        base_color: Color,
        hover_color: Color,
        active_color: Color,
        corner_radius: f32,
        outline_thickness: f32,
        outline_color: Color,
        child: Child,
    ) -> UiRef {
        Self {
            base: FillStyle {
                fill: base_color,
                outline: outline_color,
                corner_radius,
                outline_thickness,
            },
            hover: FillStyle {
                fill: hover_color,
                outline: outline_color,
                corner_radius,
                outline_thickness,
            },
            active: FillStyle {
                fill: active_color,
                outline: outline_color,
                corner_radius,
                outline_thickness,
            },
            child,
        }
        .to_ref()
    }
}

impl UiNode for ActiveFill {
    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let style = if ui.is_active(area) {
            &self.active
        } else if ui.is_hovered(area) {
            &self.hover
        } else {
            &self.base
        };

        draw_rounded_rect_with_outline(
            area.top_left,
            area.size,
            style.fill,
            style.corner_radius,
            style.outline_thickness,
            style.outline,
        );

        self.child.node.draw(area, ui)
    }

    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }
}
