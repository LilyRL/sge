use sge_api::area::AreaExt;
use sge_types::Pattern;

use super::*;

#[derive(Debug)]
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

    fn size(&self, area: Area) -> Vec2 {
        self.child.node.size(area)
    }

    fn draw(&self, area: Area, state: &UiState) -> Vec2 {
        area.fill(self.color);
        self.child.node.draw(area, state)
    }
}

impl UiRef {
    pub fn fill(self, color: Color) -> UiRef {
        BoxFill::new(color, self)
    }
}

#[derive(Debug)]
pub struct HoverBoxFill {
    base_color: Color,
    hovered_color: Color,
    child: Child,
}

impl HoverBoxFill {
    pub fn new(base_color: Color, hovered_color: Color, child: Child) -> UiRef {
        Self {
            base_color,
            hovered_color,
            child,
        }
        .to_ref()
    }
}

impl UiNode for HoverBoxFill {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }

    fn size(&self, area: Area) -> Vec2 {
        self.child.node.size(area)
    }

    fn draw(&self, area: Area, state: &UiState) -> Vec2 {
        let is_hovered = state.input().is_cursor_within_area(area);
        let color = if is_hovered {
            self.hovered_color
        } else {
            self.base_color
        };
        area.fill(color);
        self.child.node.draw(area, state);
        area.size
    }
}

#[derive(Debug)]
pub struct PatternBoxFill {
    main: Color,
    alt: Color,
    pattern: Pattern,
    scale: f32,
    child: Child,
}

impl PatternBoxFill {
    pub fn new(main: Color, alt: Color, pattern: Pattern, scale: f32, child: Child) -> UiRef {
        Self {
            main,
            alt,
            pattern,
            scale,
            child,
        }
        .to_ref()
    }
}

impl UiNode for PatternBoxFill {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }

    fn size(&self, area: Area) -> Vec2 {
        self.child.node.size(area)
    }

    fn draw(&self, area: Area, state: &UiState) -> Vec2 {
        area.fill_pattern(self.main, self.alt, self.pattern, self.scale);
        self.child.node.draw(area, state)
    }
}
