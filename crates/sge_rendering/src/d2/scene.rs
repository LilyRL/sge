use crate::api::{draw_scene, draw_scene_to, draw_scene_world};

use super::*;

#[derive(Clone)]
pub struct Scene2D {
    pub(crate) draws: Vec<DrawCommand>,
}

impl Scene2D {
    pub fn empty() -> Self {
        Self { draws: vec![] }
    }

    pub fn clear(&mut self) {
        self.draws.clear();
    }

    pub fn renderer(&mut self) -> Renderer2D {
        Renderer2D {
            draws: &mut self.draws as *mut Vec<DrawCommand>,
            ty: RendererType::Scene,
        }
    }

    pub fn draw(&self) {
        draw_scene(self);
    }

    pub fn draw_world(&self) {
        draw_scene_world(self);
    }

    pub fn draw_to(&self, renderer: Renderer2D) {
        draw_scene_to(self, renderer);
    }
}
