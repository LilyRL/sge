use super::*;

pub const EMPTY: UiRef = UiRef(0);

#[derive(Debug)]
pub(crate) struct Empty;

impl UiNode for Empty {
    fn preferred_dimensions(&self) -> Vec2 {
        Vec2::ZERO
    }

    fn draw(&self, _: Area, _: &UiState) -> Vec2 {
        Vec2::ZERO
    }
}
