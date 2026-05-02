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

#[derive(Debug)]
pub struct Observer(Child);

impl Observer {
    pub fn new(child: Child) -> UiRef {
        Self(child).to_ref()
    }
}

impl UiNode for Observer {
    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let child_size = self.0.draw(area, ui);
        println!("Area: {:#?}, Child size: {:#?}", area, child_size);
        child_size
    }

    fn preferred_dimensions(&self) -> Vec2 {
        println!("Preferred dimensions called");
        self.0.preferred_dimensions()
    }

    fn size(&self, area: Area) -> Vec2 {
        let size = self.0.size(area);
        println!("Size called: {}", size);

        size
    }
}
