use sge_api::shapes_2d::Shape2DExt;

pub struct ParticleSystem {}

pub struct ParticleBatch<T: Shape2DExt> {
    rotation: f32,
    shape: T,
}
