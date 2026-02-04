use super::*;

pub struct BoxFill {
    color: Color,
    child: Child,
}

impl BoxFill {
    pub fn new(color: Color, child: Child) -> UiRef {
        Self { color, child }.to_ref()
    }
}

impl UiNode for BoxFill {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }

    fn draw(&self, area: Area, state: &UiState) -> Vec2 {
        area.fill(self.color);
        self.child.node.draw(area, state)
    }
}

pub struct HoverFill {
    base_color: Color,
    hovered_color: Color,
    child: Child,
}

impl HoverFill {
    pub fn new(base_color: Color, hovered_color: Color, child: Child) -> UiRef {
        Self {
            base_color,
            hovered_color,
            child,
        }
        .to_ref()
    }
}

impl UiNode for HoverFill {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }

    fn draw(&self, area: Area, state: &UiState) -> Vec2 {
        let is_hovered = state.input().is_cursor_within_area(area);
        let color = if is_hovered {
            self.hovered_color
        } else {
            self.base_color
        };
        area.fill(color);
        self.child.node.draw(area, state)
    }
}
