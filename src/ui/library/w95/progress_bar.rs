use super::super::*;
use super::*;

pub struct ProgressBar;

impl ProgressBar {
    pub fn new(size: Vec2, value: f32, max: f32, id: usize) -> UiRef {
        SizedBox::new(
            size,
            Card::thin(
                0.0,
                base::ProgressBar {
                    color: SECONDARY,
                    state: State::from_id(id),
                    interpolation_speed: 40.0,
                    max,
                    value,
                }
                .to_ref(),
            ),
        )
    }
}
