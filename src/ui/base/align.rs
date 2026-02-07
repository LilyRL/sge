use bevy_math::vec2;

use super::*;

#[derive(Debug)]
pub enum AlignType {
    Start,
    Center,
    End,
}

impl AlignType {
    fn align(&self, min: f32, max: f32, length: f32) -> f32 {
        match self {
            Self::Start => min,
            Self::End => max - length,
            Self::Center => (min + max - length) / 2.0,
        }
    }
}

#[derive(Debug)]
pub struct Align {
    x: AlignType,
    y: AlignType,
    child: Child,
}

impl Align {
    pub fn new(x: AlignType, y: AlignType, child: Child) -> UiRef {
        Self { x, y, child }.to_ref()
    }

    pub fn center(child: Child) -> UiRef {
        Self {
            x: AlignType::Center,
            y: AlignType::Center,
            child,
        }
        .to_ref()
    }

    pub fn top_right(child: Child) -> UiRef {
        Self {
            x: AlignType::End,
            y: AlignType::Start,
            child,
        }
        .to_ref()
    }

    pub fn top_left(child: Child) -> UiRef {
        Self {
            x: AlignType::Start,
            y: AlignType::Start,
            child,
        }
        .to_ref()
    }

    pub fn bottom_right(child: Child) -> UiRef {
        Self {
            x: AlignType::End,
            y: AlignType::End,
            child,
        }
        .to_ref()
    }

    pub fn bottom_left(child: Child) -> UiRef {
        Self {
            x: AlignType::Start,
            y: AlignType::End,
            child,
        }
        .to_ref()
    }

    pub fn top_center(child: Child) -> UiRef {
        Self {
            x: AlignType::Center,
            y: AlignType::Start,
            child,
        }
        .to_ref()
    }

    pub fn bottom_center(child: Child) -> UiRef {
        Self {
            x: AlignType::Center,
            y: AlignType::End,
            child,
        }
        .to_ref()
    }

    pub fn center_right(child: Child) -> UiRef {
        Self {
            x: AlignType::End,
            y: AlignType::Center,
            child,
        }
        .to_ref()
    }

    pub fn center_left(child: Child) -> UiRef {
        Self {
            x: AlignType::Start,
            y: AlignType::Center,
            child,
        }
        .to_ref()
    }
}

impl UiNode for Align {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        let inner_area = self.child.node.preferred_dimensions();
        let tl = area.top_left;
        let br = area.bottom_right();

        let x = self.x.align(tl.x, br.x, inner_area.x);
        let y = self.y.align(tl.y, br.y, inner_area.y);

        let child_area = Area::new(vec2(x, y), inner_area);
        self.child.node.draw(child_area, ui)
    }
}
