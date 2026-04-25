use sge_api::shapes_2d::{GradientPoint, Orientation};
use sge_exec::fs::LoadingTexture;
use sge_time::{time, time_seconds};
use sge_vectors::Vec2;

use crate::{
    UiRef,
    base::{BoxFill, Center, MultiPointGradientFill, Text},
};

pub struct AsyncImageNode;

impl AsyncImageNode {
    pub fn new(texture: &LoadingTexture, scale: Vec2) -> UiRef {
        fn loading() -> UiRef {
            let time = time();
            let seconds = time_seconds();
            let v = (time - seconds as f32) * 1.5;
            const THICKNESS: f32 = 0.5;

            MultiPointGradientFill::new(
                Orientation::Horizontal,
                vec![
                    GradientPoint::new(super::BG1, v - THICKNESS),
                    GradientPoint::new(super::BG1, THICKNESS * 2. / 3.),
                    GradientPoint::new(super::BG3, THICKNESS * 2. / 3.),
                    GradientPoint::new(super::BG1, THICKNESS * 2. / 3.),
                    GradientPoint::new(super::BG1, 1.0 - v - THICKNESS),
                ],
            )
            .scissored()
        }

        fn error(msg: String) -> UiRef {
            BoxFill::new(
                super::BG1,
                Center::new(Text::nowrap_with_color(msg, super::ERROR)),
            )
        }

        crate::base::AsyncImageNode::new(texture, scale, loading, error)
    }
}
