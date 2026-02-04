use crate::prelude::TextureRef;

use super::*;

pub enum ImageSource {
    Texture(TextureRef),
}

pub struct ImageNode {
    pub source: ImageSource,
}

impl ImageNode {
    pub fn from_texture(texture: TextureRef) -> UiRef {
        Self {
            source: ImageSource::Texture(texture),
        }
        .to_ref()
    }
}

impl UiNode for ImageNode {
    fn preferred_dimensions(&self) -> Vec2 {
        match &self.source {
            ImageSource::Texture(texture) => texture.dimensions().as_vec2(),
        }
    }

    fn draw(&self, mut area: Area, _: &UiState) -> Vec2 {
        area.size = area.size.min(self.preferred_dimensions());

        // let normalized_dimensions = match &self.source {
        //     ImageSource::Texture(texture) => texture.normalized_dimensions(),
        // };
        // area.size = (area.size.x.min(area.size.y)) * normalized_dimensions;

        match &self.source {
            ImageSource::Texture(texture) => area.draw_texture(*texture),
        }

        area.size
    }
}
