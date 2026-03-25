use core::f32;

use bevy_math::vec2;

use super::*;

/// be careful with what order you place sized boxes and other nodes
///
/// for example, always place a border node inside a sized box and not the other way around
#[derive(Debug)]
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

    pub fn grow(child: Child) -> UiRef {
        Self {
            dimensions: Vec2::INFINITY,
            child,
        }
        .to_ref()
    }
}

impl UiRef {
    pub fn sized_wh(self, width: f32, height: f32) -> UiRef {
        SizedBox::wh(width, height, self)
    }

    pub fn sized(self, dimensions: Vec2) -> UiRef {
        SizedBox::new(dimensions, self)
    }

    pub fn height(self, height: f32) -> UiRef {
        SizedBox::height(height, self)
    }

    pub fn width(self, width: f32) -> UiRef {
        SizedBox::width(width, self)
    }
}

fn inf_to_zero(n: f32) -> f32 {
    match n {
        f32::INFINITY => 0.0,
        _ => n,
    }
}

impl UiNode for SizedBox {
    fn preferred_dimensions(&self) -> Vec2 {
        vec2(
            inf_to_zero(self.dimensions.x),
            inf_to_zero(self.dimensions.y),
        )
    }

    fn size(&self, area: Area) -> Vec2 {
        self.dimensions.min(area.size)
    }

    fn draw(&self, mut area: Area, state: &UiState) -> Vec2 {
        let dimensions = self.dimensions.min(area.size);
        area.size = dimensions;
        self.child.node.draw(area, state).max(dimensions)
    }
}

#[derive(Debug)]
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

    pub fn grow_x(child: Child) -> UiRef {
        Self {
            max_size: Vec2::new(f32::INFINITY, f32::INFINITY),
            min_size: Vec2::new(f32::INFINITY, 0.0),
            child,
        }
        .to_ref()
    }

    pub fn grow_y(child: Child) -> UiRef {
        Self {
            max_size: Vec2::new(f32::INFINITY, f32::INFINITY),
            min_size: Vec2::new(0.0, f32::INFINITY),
            child,
        }
        .to_ref()
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

impl UiRef {
    pub fn min_height(self, min_height: f32) -> UiRef {
        ConstrainedBox::min_height(min_height, self)
    }

    pub fn min_width(self, min_width: f32) -> UiRef {
        ConstrainedBox::min_width(min_width, self)
    }

    pub fn max_height(self, max_height: f32) -> UiRef {
        ConstrainedBox::max_height(max_height, self)
    }

    pub fn max_width(self, max_width: f32) -> UiRef {
        ConstrainedBox::max_width(max_width, self)
    }

    pub fn min_size(self, min_size: Vec2) -> UiRef {
        ConstrainedBox::min_size(min_size, self)
    }

    pub fn max_size(self, max_size: Vec2) -> UiRef {
        ConstrainedBox::max_size(max_size, self)
    }

    pub fn grow_x(self) -> UiRef {
        ConstrainedBox::grow_x(self)
    }

    pub fn grow_y(self) -> UiRef {
        ConstrainedBox::grow_y(self)
    }
}

#[derive(Debug)]
pub struct EmptyBox {
    height: f32,
    width: f32,
}

impl EmptyBox {
    pub fn new(width: f32, height: f32) -> UiRef {
        Self { width, height }.to_ref()
    }

    pub fn height(height: f32) -> UiRef {
        Self { width: 0.0, height }.to_ref()
    }

    pub fn width(width: f32) -> UiRef {
        Self { width, height: 0.0 }.to_ref()
    }
}

impl UiNode for EmptyBox {
    fn preferred_dimensions(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }

    fn size(&self, area: Area) -> Vec2 {
        self.preferred_dimensions().max(area.size)
    }

    fn draw(&self, _: Area, _: &UiState) -> Vec2 {
        self.preferred_dimensions()
    }
}
