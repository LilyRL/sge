use core::f32;

use super::*;

pub struct SizedBox {
    pub dimensions: Vec2,
    pub child: Child,
}

impl SizedBox {
    pub fn new(dimensions: Vec2, child: Child) -> UiRef {
        Self { dimensions, child }.to_ref()
    }

    pub fn zero(child: Child) -> UiRef {
        Self {
            dimensions: Vec2::ZERO,
            child,
        }
        .to_ref()
    }

    /// has infinite height
    pub fn width(width: f32, child: Child) -> UiRef {
        Self {
            dimensions: Vec2::new(width, f32::INFINITY),
            child,
        }
        .to_ref()
    }

    /// has infinite width
    pub fn height(height: f32, child: Child) -> UiRef {
        Self {
            dimensions: Vec2::new(f32::INFINITY, height),
            child,
        }
        .to_ref()
    }

    pub fn wh(width: f32, height: f32, child: Child) -> UiRef {
        Self {
            dimensions: Vec2::new(width, height),
            child,
        }
        .to_ref()
    }
}

impl UiNode for SizedBox {
    fn preferred_dimensions(&self) -> Vec2 {
        self.dimensions
    }

    fn draw(&self, mut area: Area, state: &UiState) -> Vec2 {
        let dimensions = self.dimensions.min(area.size);
        area.size = dimensions;
        self.child.node.draw(area, state).max(dimensions)
    }
}

pub struct ConstrainedBox {
    max_size: Vec2,
    min_size: Vec2,
    child: Child,
}

impl ConstrainedBox {
    pub fn max_height(max_height: f32, child: Child) -> UiRef {
        Self {
            max_size: Vec2::new(f32::INFINITY, max_height),
            min_size: Vec2::ZERO,
            child,
        }
        .to_ref()
    }

    pub fn max_width(max_width: f32, child: Child) -> UiRef {
        Self {
            max_size: Vec2::new(max_width, f32::INFINITY),
            min_size: Vec2::ZERO,
            child,
        }
        .to_ref()
    }

    pub fn min_height(min_height: f32, child: Child) -> UiRef {
        Self {
            max_size: Vec2::new(f32::INFINITY, f32::INFINITY),
            min_size: Vec2::new(0.0, min_height),
            child,
        }
        .to_ref()
    }

    pub fn min_width(min_width: f32, child: Child) -> UiRef {
        Self {
            max_size: Vec2::new(f32::INFINITY, f32::INFINITY),
            min_size: Vec2::new(min_width, 0.0),
            child,
        }
        .to_ref()
    }

    pub fn min_size(min_size: Vec2, child: Child) -> UiRef {
        Self {
            max_size: Vec2::new(f32::INFINITY, f32::INFINITY),
            min_size,
            child,
        }
        .to_ref()
    }

    pub fn max_size(max_size: Vec2, child: Child) -> UiRef {
        Self {
            max_size,
            min_size: Vec2::ZERO,
            child,
        }
        .to_ref()
    }

    pub fn new(min_size: Vec2, max_size: Vec2, child: Child) -> UiRef {
        Self {
            max_size,
            min_size,
            child,
        }
        .to_ref()
    }

    fn transform_size(&self, size: Vec2) -> Vec2 {
        size.clamp(self.min_size, self.max_size)
    }
}

impl UiNode for ConstrainedBox {
    fn preferred_dimensions(&self) -> Vec2 {
        self.transform_size(self.child.node.preferred_dimensions())
    }

    fn draw(&self, mut area: Area, state: &UiState) -> Vec2 {
        area.size = self.transform_size(area.size);
        self.child.node.draw(area, state)
    }
}
