use easy_ext::ext;
use sge_color::Color;
use sge_rendering::api::draw_texture_scaled;
use sge_shapes::d2::Rect;
use sge_textures::TextureRef;
use sge_types::{Area, Pattern};

use crate::shapes_2d::{draw_rect, draw_shape_with_pattern};

#[ext(AreaExt)]
pub impl Area {
    fn fill(&self, color: Color) {
        draw_rect(self.top_left, self.size, color);
    }

    fn fill_pattern(&self, main: Color, alt: Color, pattern: Pattern, scale: f32) {
        let rect = Rect::new(self.top_left, self.size, main);
        draw_shape_with_pattern(rect, alt, pattern, scale);
    }

    fn draw_texture(&self, texture: TextureRef) {
        draw_texture_scaled(texture, self.top_left, self.size);
    }
}
