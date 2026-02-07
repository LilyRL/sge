use super::*;

#[derive(Debug)]
pub struct Center {
    vertical: bool,
    horizontal: bool,
    child: Child,
}

impl Center {
    pub fn new(child: Child) -> UiRef {
        Self {
            vertical: true,
            horizontal: true,
            child,
        }
        .to_ref()
    }

    pub fn vertical(child: Child) -> UiRef {
        Self {
            vertical: true,
            horizontal: false,
            child,
        }
        .to_ref()
    }

    pub fn horizontal(child: Child) -> UiRef {
        Self {
            vertical: false,
            horizontal: true,
            child,
        }
        .to_ref()
    }
}

impl UiNode for Center {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }

    fn draw(&self, mut area: Area, ui: &UiState) -> Vec2 {
        let inner = self.preferred_dimensions();
        let diff = (area.size - inner).max(Vec2::ZERO);

        if self.vertical {
            area.top_left.y += diff.y / 2.0;
        }

        if self.horizontal {
            area.top_left.x += diff.x / 2.0;
        }

        self.child.node.draw(area, ui)
    }
}
