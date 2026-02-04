use bevy_math::vec2;

use crate::prelude::lerp;

use super::*;

pub struct ProgressBar {
    max: f32,
    value: f32,
    color: Color,
    state: State<ProgressBarState>,
    interpolation_speed: f32,
}

#[derive(Default)]
struct ProgressBarState {
    displayed_value: f32,
}

impl ProgressBar {
    pub fn new(value: f32, max: f32, color: Color, id: usize) -> UiRef {
        ProgressBar {
            max,
            value,
            color,
            state: State::from_id(id),
            interpolation_speed: 10.0,
        }
        .to_ref()
    }

    /// default is 10
    pub fn new_with_interpolation_speed(
        value: f32,
        max: f32,
        color: Color,
        interpolation_speed: f32,
        id: usize,
    ) -> UiRef {
        ProgressBar {
            max,
            value,
            color,
            state: State::from_id(id),
            interpolation_speed,
        }
        .to_ref()
    }
}

impl UiNode for ProgressBar {
    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let state = self.state.get_or_default();

        state.displayed_value = lerp(
            state.displayed_value,
            self.value,
            self.interpolation_speed * ui.delta_time,
        );
        let ratio = state.displayed_value / self.max;
        let width = area.width() * ratio;

        draw_rect(area.top_left, vec2(width, area.height()), self.color);

        area.size
    }

    fn preferred_dimensions(&self) -> Vec2 {
        Vec2::new(200.0, 20.0)
    }
}
