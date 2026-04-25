use sge_api::shapes_2d::{
    GradientPoint, Orientation, draw_multi_point_gradient, draw_rect_gradient,
};

use super::*;

#[derive(Debug)]
pub struct GradientFill {
    top_left: Color,
    top_right: Color,
    bottom_left: Color,
    bottom_right: Color,
    child: Child,
}

impl GradientFill {
    pub fn top_to_bottom(top: Color, bottom: Color, child: impl Into<Child>) -> UiRef {
        Self {
            top_left: top,
            top_right: top,
            bottom_left: bottom,
            bottom_right: bottom,
            child: child.into(),
        }
        .to_ref()
    }

    pub fn bottom_to_top(bottom: Color, top: Color, child: impl Into<Child>) -> UiRef {
        Self {
            top_left: top,
            top_right: top,
            bottom_left: bottom,
            bottom_right: bottom,
            child: child.into(),
        }
        .to_ref()
    }

    pub fn left_to_right(left: Color, right: Color, child: impl Into<Child>) -> UiRef {
        Self {
            top_left: left,
            top_right: right,
            bottom_left: left,
            bottom_right: right,
            child: child.into(),
        }
        .to_ref()
    }

    pub fn right_to_left(right: Color, left: Color, child: impl Into<Child>) -> UiRef {
        Self {
            top_left: left,
            top_right: right,
            bottom_left: left,
            bottom_right: right,
            child: child.into(),
        }
        .to_ref()
    }

    pub fn top_right_to_bottom_left(
        top_right: Color,
        bottom_left: Color,
        child: impl Into<Child>,
    ) -> UiRef {
        let blend = top_right.blend_halfway(bottom_left);
        Self {
            top_left: blend,
            top_right,
            bottom_left,
            bottom_right: blend,
            child: child.into(),
        }
        .to_ref()
    }

    pub fn top_left_to_bottom_right(
        top_left: Color,
        bottom_right: Color,
        child: impl Into<Child>,
    ) -> UiRef {
        let blend = top_left.blend_halfway(bottom_right);
        Self {
            top_left,
            top_right: blend,
            bottom_left: blend,
            bottom_right,
            child: child.into(),
        }
        .to_ref()
    }

    pub fn bottom_left_to_top_right(
        bottom_left: Color,
        top_right: Color,
        child: impl Into<Child>,
    ) -> UiRef {
        let blend = bottom_left.blend_halfway(top_right);
        Self {
            top_left: blend,
            top_right,
            bottom_left,
            bottom_right: blend,
            child: child.into(),
        }
        .to_ref()
    }

    pub fn bottom_right_to_top_left(
        bottom_right: Color,
        top_left: Color,
        child: impl Into<Child>,
    ) -> UiRef {
        let blend = bottom_right.blend_halfway(top_left);
        Self {
            top_left,
            top_right: blend,
            bottom_left: blend,
            bottom_right,
            child: child.into(),
        }
        .to_ref()
    }

    pub fn four_corners(
        top_left: Color,
        top_right: Color,
        bottom_left: Color,
        bottom_right: Color,
        child: impl Into<Child>,
    ) -> UiRef {
        Self {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            child: child.into(),
        }
        .to_ref()
    }
}

impl UiNode for GradientFill {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        draw_rect_gradient(
            area.top_left,
            area.size,
            self.top_left,
            self.top_right,
            self.bottom_left,
            self.bottom_right,
        );

        self.child.node.draw(area, ui)
    }
}

#[derive(Debug)]
pub struct MultiPointGradientFill {
    orientation: Orientation,
    points: Vec<GradientPoint>,
}

impl MultiPointGradientFill {
    pub fn new(orientation: Orientation, points: Vec<GradientPoint>) -> UiRef {
        Self {
            orientation,
            points,
        }
        .to_ref()
    }
}

impl UiNode for MultiPointGradientFill {
    fn preferred_dimensions(&self) -> Vec2 {
        Vec2::ZERO
    }

    fn size(&self, area: Area) -> Vec2 {
        area.size
    }

    fn draw(&self, area: Area, _: &UiState) -> Vec2 {
        draw_multi_point_gradient(area.top_left, area.size, self.orientation, &self.points);
        area.size
    }
}
