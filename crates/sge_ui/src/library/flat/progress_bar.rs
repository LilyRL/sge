use super::super::{UiRef, *};

pub struct ProgressBar;

impl ProgressBar {
    pub fn new(size: Vec2, color: Color, value: f32, max: f32, id: usize) -> UiRef {
        SizedBox::new(
            size,
            Border::all(
                4.0,
                color,
                base::ProgressBar::new(value, max, BoxFill::new(color, EMPTY), id),
            ),
        )
    }

    pub fn rounded(
        size: Vec2,
        color: Color,
        value: f32,
        max: f32,
        radius: f32,
        id: usize,
    ) -> UiRef {
        SizedBox::new(
            size,
            Border::all(
                4.0,
                color,
                base::ProgressBar::new(value, max, RoundedFill::new(color, radius, EMPTY), id),
            ),
        )
    }

    pub fn primary(size: Vec2, value: f32, max: f32, id: usize) -> UiRef {
        Self::new(size, super::PRIMARY_TEXT_COLOR, value, max, id)
    }
}
