use sge_api::area::AreaExt;
use sge_exec::fs::LoadingTexture;
use sge_textures::TextureRef;

use super::*;

#[derive(Debug)]
pub enum ImageSource {
    Texture(TextureRef),
}

#[derive(Debug)]
pub struct ImageNode {
    source: ImageSource,
}

impl ImageNode {
    pub fn from_texture(texture: TextureRef) -> UiRef {
        Self {
            source: ImageSource::Texture(texture),
        }
        .to_ref()
    }

    pub fn from_texture_with_scale(texture: TextureRef, scale: f32) -> UiRef {
        Self::from_texture(texture).sized(texture.normalized_dimensions * scale)
    }
}

impl UiNode for ImageNode {
    fn preferred_dimensions(&self) -> Vec2 {
        match &self.source {
            ImageSource::Texture(texture) => texture.dimensions.as_vec2(),
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

pub enum AsyncImageSource {
    Texture(*const LoadingTexture),
}

impl Debug for AsyncImageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Texture(_) => f.write_str("Texture"),
        }
    }
}

#[derive(Debug)]
pub struct AsyncImageNode {
    source: AsyncImageSource,
    loading: fn() -> Child,
    error: fn(String) -> Child,
    scale: Vec2,
}

impl AsyncImageNode {
    pub fn new(
        texture: &LoadingTexture,
        scale: Vec2,
        loading: fn() -> Child,
        error: fn(String) -> Child,
    ) -> UiRef {
        Self {
            source: AsyncImageSource::Texture(texture as *const LoadingTexture),
            loading,
            error,
            scale,
        }
        .to_ref()
    }
}

impl UiNode for AsyncImageNode {
    fn preferred_dimensions(&self) -> Vec2 {
        self.scale
    }

    fn size(&self, _area: Area) -> Vec2 {
        self.scale
    }

    fn draw(&self, mut area: Area, ui: &UiState) -> Vec2 {
        area.size = area.size.min(self.scale);
        match &self.source {
            AsyncImageSource::Texture(texture) => match unsafe { &**texture } {
                None => {
                    (self.loading)().draw(area, ui);
                }
                Some(Ok(tex)) => {
                    area.draw_texture(*tex);
                }
                Some(Err(e)) => {
                    (self.error)(e.to_string()).draw(area, ui);
                }
            },
        }
        area.size
    }
}
