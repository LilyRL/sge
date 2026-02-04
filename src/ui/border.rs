use crate::prelude::draw_custom_shape;

use super::*;

#[derive(Clone, Copy)]
pub struct BorderStyle {
    thickness: f32,
    color: Color,
}

enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

impl BorderStyle {
    pub fn new(thickness: f32, color: Color) -> Self {
        Self { thickness, color }
    }

    fn draw(&self, a: Vec2, b: Vec2, side: Side) {
        let ((x_a, y_a), (x_b, y_b)) = match side {
            Side::Top => ((1, 1), (-1, 1)),
            Side::Left => ((1, 1), (1, -1)),
            Side::Bottom => ((1, -1), (-1, -1)),
            Side::Right => ((-1, 1), (-1, -1)),
        };

        let mul_a = Vec2::new(x_a as f32, y_a as f32);
        let mul_b = Vec2::new(x_b as f32, y_b as f32);

        let c = a + (self.thickness * mul_a);
        let d = b + (self.thickness * mul_b);

        draw_custom_shape(vec![a, c, d, b], self.color);
    }
}

pub struct Border {
    top: BorderStyle,
    bottom: BorderStyle,
    left: BorderStyle,
    right: BorderStyle,

    child: Child,
}

impl Border {
    pub fn tblr(
        top: BorderStyle,
        bottom: BorderStyle,
        left: BorderStyle,
        right: BorderStyle,
        child: Child,
    ) -> UiRef {
        Self {
            top,
            bottom,
            left,
            right,
            child,
        }
        .to_ref()
    }

    pub fn all_style(style: BorderStyle, child: Child) -> UiRef {
        Self {
            top: style,
            bottom: style,
            left: style,
            right: style,
            child,
        }
        .to_ref()
    }

    pub fn all(thickness: f32, color: Color, child: Child) -> UiRef {
        let style = BorderStyle { thickness, color };
        Self {
            top: style,
            bottom: style,
            left: style,
            right: style,
            child,
        }
        .to_ref()
    }

    fn extra_size(&self) -> Vec2 {
        Vec2::new(
            self.left.thickness + self.right.thickness,
            self.top.thickness + self.bottom.thickness,
        )
    }

    fn offset(&self) -> Vec2 {
        Vec2::new(self.left.thickness, self.top.thickness)
    }
}

impl UiNode for Border {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions() + self.extra_size()
    }

    fn draw(&self, mut area: Area, state: &UiState) -> Vec2 {
        self.top.draw(area.top_left(), area.top_right(), Side::Top);
        self.top
            .draw(area.top_left(), area.bottom_left(), Side::Left);
        self.bottom
            .draw(area.bottom_left(), area.bottom_right(), Side::Bottom);
        self.right
            .draw(area.top_right(), area.bottom_right(), Side::Right);

        area.size -= self.extra_size();
        area.top_left += self.offset();

        let dimensions = self.child.node.draw(area, state);
        dimensions + self.extra_size()
    }
}
