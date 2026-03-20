use bevy_math::{Vec2, Vec3, Vec4};
use global::global;
use rand::{
    Rng,
    distr::{
        Distribution, StandardUniform,
        uniform::{SampleRange, SampleUniform},
    },
    rng,
    rngs::ThreadRng,
};
use sge_color::Color;

pub struct RandomState {
    rng: ThreadRng,
    counter: usize,
}

global!(RandomState, random);

pub fn init() {
    set_random(RandomState {
        rng: rng(),
        counter: 0,
    });
    log::info!("Initialized rng");
}

pub fn rand<T>() -> T
where
    StandardUniform: Distribution<T>,
{
    get_random().rng.random()
}

pub fn rand_usize() -> usize {
    get_random().rng.random::<u64>() as usize
}

/// Return a bool with a probability `p` of being true.
pub fn rand_bool(p: f64) -> bool {
    get_random().rng.random_bool(p)
}

pub fn rand_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    get_random().rng.random_range(range)
}

/// Return a bool with a probability of `numerator/denominator` of being
/// true.
pub fn rand_ratio(numerator: u32, denominator: u32) -> bool {
    get_random().rng.random_ratio(numerator, denominator)
}

pub fn get_next_counter() -> usize {
    let state = get_random();
    let n = state.counter;
    state.counter += 1;
    n
}

pub fn rand_color() -> Color {
    Color::new(rand(), rand(), rand())
}

pub fn rand_vec2() -> Vec2 {
    Vec2::new(rand_range(-1.0..1.0), rand_range(-1.0..1.0))
}

pub fn rand_vec3() -> Vec3 {
    Vec3::new(
        rand_range(-1.0..1.0),
        rand_range(-1.0..1.0),
        rand_range(-1.0..1.0),
    )
}

pub fn rand_vec4() -> Vec4 {
    Vec4::new(
        rand_range(-1.0..1.0),
        rand_range(-1.0..1.0),
        rand_range(-1.0..1.0),
        rand_range(-1.0..1.0),
    )
}
