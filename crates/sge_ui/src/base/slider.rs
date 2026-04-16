use super::*;
use crate::NumberValue;
use core::f32;
use sge_input::{cursor, mouse_pressed, mouse_released};
use sge_utils::{FromF32, PartialClamp, ToF32};
use sge_vectors::vec2;
use sge_window::{use_grab_cursor_icon, use_grabbing_cursor_icon};

#[derive(Debug)]
/// If you're confused on how to use the slider element, check the source for the flat Slider component
pub struct Slider<T: NumberValue> {
    min: T,
    max: T,
    value: *mut T,
    handle: Child,
    bar: Child,
    state: State<SliderState>,
}

#[derive(Default, Debug)]
pub struct SliderState {
    captured: bool,
}

impl<T: NumberValue> Slider<T> {
    pub fn new(value: &mut T, min: T, max: T, handle: Child, bar: Child, id: usize) -> UiRef {
        Self {
            min,
            max,
            value: value as *mut T,
            state: State::from_id(id),
            handle,
            bar,
        }
        .to_ref()
    }
}

impl<T: NumberValue> UiNode for Slider<T> {
    fn preferred_dimensions(&self) -> Vec2 {
        let height = self.bar.preferred_dimensions().y;
        vec2(f32::INFINITY, height)
    }

    fn size(&self, _: Area) -> Vec2 {
        self.preferred_dimensions()
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let state = self.state.get_or_default();
        let value = unsafe { &mut *self.value };
        let handle_size = self.handle.preferred_dimensions();
        let bar_size = vec2(area.width(), self.bar.preferred_dimensions().y);
        let size = bar_size.max(handle_size);

        let width = self.max - self.min;

        let usable_width = area.width() - handle_size.x;

        let cursor_ratio = |cursor_x: f32| {
            let relative_x = cursor_x - area.top_left.x - handle_size.x / 2.0;
            (relative_x / usable_width).clamp(0.0, 1.0)
        };

        let set_value_from_cursor = |cursor_x: f32| {
            let ratio = cursor_ratio(cursor_x);
            T::from_f32(self.min.to_f32() + ratio * width.to_f32())
        };

        let ratio = (value.to_f32() - self.min.to_f32()) / width.to_f32();
        let x_pos = ratio * usable_width;

        let handle_area = Area::new(area.top_left + vec2(x_pos, 0.0), handle_size);
        let bar_area = Area::new(area.top_left, bar_size);

        let is_handle_hovered = ui.is_hovered(handle_area);
        let is_bar_hovered = ui.is_hovered(bar_area);

        if mouse_released(MouseButton::Left) {
            state.captured = false;
        }

        if mouse_pressed(MouseButton::Left) && (is_bar_hovered || is_handle_hovered) {
            state.captured = true;
            if let Some(cursor_pos) = cursor() {
                *value = set_value_from_cursor(cursor_pos.x);
            }
        }

        if state.captured {
            if let Some(cursor_pos) = cursor() {
                *value = set_value_from_cursor(cursor_pos.x);
            }
            use_grabbing_cursor_icon();
        } else if is_handle_hovered || is_bar_hovered {
            use_grab_cursor_icon();
        }

        if !state.captured && (is_handle_hovered || is_bar_hovered) {
            use_grab_cursor_icon();
        }

        self.bar.draw(bar_area, ui);
        self.handle.draw(handle_area, ui);
        size
    }
}
