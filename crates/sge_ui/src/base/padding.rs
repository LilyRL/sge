use super::*;

#[derive(Default, Debug)]
pub struct Padding {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,

    child: Child,
}

impl Padding {
    pub fn tblr(top: f32, bottom: f32, left: f32, right: f32, child: Child) -> UiRef {
        Self {
            top,
            bottom,
            left,
            right,
            child,
        }
        .to_ref()
    }

    pub fn left_right(left: f32, right: f32, child: Child) -> UiRef {
        Self {
            top: 0.0,
            bottom: 0.0,
            left,
            right,
            child,
        }
        .to_ref()
    }

    pub fn top_bottom(top: f32, bottom: f32, child: Child) -> UiRef {
        Self {
            top,
            bottom,
            left: 0.0,
            right: 0.0,
            child,
        }
        .to_ref()
    }

    pub fn top(top: f32, child: Child) -> UiRef {
        Self {
            top,
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
            child,
        }
        .to_ref()
    }

    pub fn bottom(bottom: f32, child: Child) -> UiRef {
        Self {
            top: 0.0,
            bottom,
            left: 0.0,
            right: 0.0,
            child,
        }
        .to_ref()
    }

    pub fn left(left: f32, child: Child) -> UiRef {
        Self {
            top: 0.0,
            bottom: 0.0,
            left,
            right: 0.0,
            child,
        }
        .to_ref()
    }

    pub fn right(right: f32, child: Child) -> UiRef {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right,
            child,
        }
        .to_ref()
    }

    pub fn all(padding: f32, child: Child) -> UiRef {
        Self {
            top: padding,
            bottom: padding,
            left: padding,
            right: padding,
            child,
        }
        .to_ref()
    }

    pub fn xy(x: f32, y: f32, child: Child) -> UiRef {
        Self {
            top: y,
            bottom: y,
            left: x,
            right: x,
            child,
        }
        .to_ref()
    }

    pub fn zero(child: Child) -> UiRef {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
            child,
        }
        .to_ref()
    }

    pub fn horizontal(padding: f32, child: Child) -> UiRef {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: padding,
            right: padding,
            child,
        }
        .to_ref()
    }

    pub fn vertical(padding: f32, child: Child) -> UiRef {
        Self {
            top: padding,
            bottom: padding,
            left: 0.0,
            right: 0.0,
            child,
        }
        .to_ref()
    }

    fn extra_size(&self) -> Vec2 {
        Vec2::new(self.left + self.right, self.top + self.bottom)
    }

    fn offset(&self) -> Vec2 {
        Vec2::new(self.left, self.top)
    }
}

impl UiNode for Padding {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions() + self.extra_size()
    }

    fn size(&self, area: Area) -> Vec2 {
        let child_area = Area {
            top_left: area.top_left + self.offset(),
            size: area.size - self.extra_size(),
        };
        self.child.node.size(child_area) + self.extra_size()
    }

    fn draw(&self, mut area: Area, state: &UiState) -> Vec2 {
        area.size -= self.extra_size();
        area.top_left += self.offset();
        self.child.node.draw(area, state) + self.extra_size()
    }
}

impl UiRef {
    pub fn padding(self, padding: f32) -> UiRef {
        Padding::all(padding, self)
    }

    pub fn padding_xy(self, x: f32, y: f32) -> UiRef {
        Padding::xy(x, y, self)
    }

    pub fn padding_horizontal(self, padding: f32) -> UiRef {
        Padding::horizontal(padding, self)
    }

    pub fn padding_vertical(self, padding: f32) -> UiRef {
        Padding::vertical(padding, self)
    }

    pub fn padding_tblr(self, top: f32, bottom: f32, left: f32, right: f32) -> UiRef {
        Padding::tblr(top, bottom, left, right, self)
    }

    pub fn padding_top(self, top: f32) -> UiRef {
        Padding::top(top, self)
    }

    pub fn padding_bottom(self, bottom: f32) -> UiRef {
        Padding::bottom(bottom, self)
    }

    pub fn padding_left(self, left: f32) -> UiRef {
        Padding::left(left, self)
    }

    pub fn padding_right(self, right: f32) -> UiRef {
        Padding::right(right, self)
    }
}
