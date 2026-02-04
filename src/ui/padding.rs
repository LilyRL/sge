use super::*;

#[derive(Default)]
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

    fn draw(&self, mut area: Area, state: &UiState) -> Vec2 {
        area.size -= self.extra_size();
        area.top_left += self.offset();
        self.child.node.draw(area, state) + self.extra_size()
    }
}
