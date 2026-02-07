use super::super::{State, UiRef, *};

pub struct ProgressBar;

impl ProgressBar {
    pub fn new(size: Vec2, color: Color, value: f32, max: f32, id: usize) -> UiRef {
        SizedBox::new(
            size,
            Border::all(
                4.0,
                color,
                base::ProgressBar {
                    color,
                    state: State::from_id(id),
                    interpolation_speed: 10.0,
                    max,
                    value,
                }
                .to_ref(),
            ),
        )
    }
}
