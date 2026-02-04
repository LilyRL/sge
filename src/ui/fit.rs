use super::*;

pub struct Fit {
    child: Child,
}

impl Fit {
    pub fn new(child: Child) -> UiRef {
        Self { child }.to_ref()
    }
}

impl UiNode for Fit {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }

    fn draw(&self, mut area: Area, state: &UiState) -> Vec2 {
        area.size = area.size.min(self.preferred_dimensions());
        self.child.node.draw(area, state)
    }
}
