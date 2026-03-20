use bevy_math::Vec2;

pub mod collision;
pub mod transform;
pub mod usize_rect;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub trait Vec2Ext {
    fn invert_y(&self) -> Self;
    fn invert_x(&self) -> Self;
}

impl Vec2Ext for Vec2 {
    fn invert_y(&self) -> Self {
        Vec2::new(self.x, -self.y)
    }

    fn invert_x(&self) -> Self {
        Vec2::new(-self.x, self.y)
    }
}
