use bevy_math::vec2;
use sge_api::Drawable;
use sge_shapes::d2::Circle;

use super::*;

#[derive(Debug)]
pub struct CircleFill {
    color: Color,
}

impl CircleFill {
    pub fn new(color: Color) -> UiRef {
        Self { color }.to_ref()
    }
}

impl UiNode for CircleFill {
    fn preferred_dimensions(&self) -> Vec2 {
        vec2(50.0, 50.0)
    }

    fn draw(&self, area: Area, _: &UiState) -> Vec2 {
        Circle::from_top_left(area.top_left, area.size / 2.0, self.color).draw();

        area.size
    }
}
