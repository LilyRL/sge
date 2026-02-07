use super::*;

#[derive(Debug)]
pub struct Stack {
    size: Vec2,
    children: Vec<Child>,
}

impl Stack {
    pub fn new(size: Vec2, children: impl Into<Vec<Child>>) -> UiRef {
        Self {
            size,
            children: children.into(),
        }
        .to_ref()
    }
}

impl UiNode for Stack {
    fn preferred_dimensions(&self) -> Vec2 {
        self.size
    }

    fn draw(&self, mut area: Area, ui: &UiState) -> Vec2 {
        let dimensions = self.size.min(area.size);
        area.size = dimensions;

        for child in &self.children {
            child.node.draw(area, ui);
        }

        self.size
    }
}
