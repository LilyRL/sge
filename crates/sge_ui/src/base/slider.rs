use core::f32;
use std::ops::{Add, Div, Mul, Sub};

use bevy_math::vec2;
use sge_input::{cursor_diff, mouse_pressed, mouse_released};
use sge_utils::{FromF32, PartialClamp, ToF32};
use sge_window::{use_default_cursor_icon, use_grab_cursor_icon, use_grabbing_cursor_icon};

use super::*;

pub trait SliderValue = Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Div<Self, Output = Self>
    + Mul<Self, Output = Self>
    + 'static
    + Sized
    + Debug
    + PartialOrd
    + Copy
    + ToF32
    + FromF32
    + PartialClamp;

#[derive(Debug)]
pub struct Slider<T: SliderValue> {
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
    f32_value: f32,
    initialised: bool,
}

impl<T: SliderValue> Slider<T> {
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

impl<T: SliderValue> UiNode for Slider<T> {
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
        // FIXME: hacky
        let bar_size = vec2(area.width(), self.bar.preferred_dimensions().y);

        if !state.initialised || !state.captured {
            state.f32_value = value.to_f32();
            state.initialised = true;
        }

        let width = self.max - self.min;
        let ratio = (value.to_f32() - self.min.to_f32()) / width.to_f32();
        let x_pos = ratio * (area.width() - handle_size.x);
        let size = bar_size.max(handle_size);

        let handle_area = Area::new(area.top_left + vec2(x_pos, 0.0), handle_size);
        let bar_area = Area::new(area.top_left, bar_size);
        let is_hovered = ui.is_hovered(handle_area);

        if state.captured {
            let delta = cursor_diff().x;
            let ratio = delta / area.width();
            let delta_value = ratio * width.to_f32();
            state.f32_value =
                (state.f32_value + delta_value).clamp(self.min.to_f32(), self.max.to_f32());
            *value = T::from_f32(state.f32_value);
            use_grabbing_cursor_icon();
        }

        if mouse_pressed(MouseButton::Left) {
            state.captured = is_hovered;
        }

        if mouse_released(MouseButton::Left) {
            state.captured = false;
            use_default_cursor_icon();
        }

        if !state.captured && is_hovered {
            use_grab_cursor_icon();
        }

        self.bar.draw(bar_area, ui);
        self.handle.draw(handle_area, ui);

        size
    }
}
