use sge_debugging::avg_fps;

use super::*;

#[derive(Debug)]
pub struct DebugNode;

impl DebugNode {
    pub fn new() -> UiRef {
        Self.to_ref()
    }
}

impl UiNode for DebugNode {
    fn preferred_dimensions(&self) -> sge_vectors::Vec2 {
        Vec2::ZERO
    }

    fn size(&self, _: sge_types::Area) -> Vec2 {
        Vec2::ZERO
    }

    fn draw(&self, area: sge_types::Area, ui: &crate::UiState) -> Vec2 {
        Text::mono_nowrap(format!("{:#?}", area)).draw(area, ui)
    }
}

pub struct FpsNode;

impl FpsNode {
    pub fn new() -> UiRef {
        Text::mono_nowrap(format!("FPS: {}", avg_fps()))
    }
}
