use std::ops::{Deref, DerefMut};

use bevy_ecs::system::ScheduleSystem;
use bevy_ecs::world::World as BWorld;
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
use sge_api::shapes_2d::Shape2DExt;
use sge_error_union::wrapper;
use sge_macros::gen_ref_type;
use sge_time::physics_delta_time;
use sge_vectors::Vec2;

pub use bevy_ecs;

#[wrapper(new(x: f32, y: f32))]
#[derive(Clone, Copy, Component, Default)]
pub struct Velocity2D(pub Vec2);

#[wrapper(new(x: f32, y: f32))]
#[derive(Clone, Copy, Component, Default)]
pub struct Position2D(pub Vec2);

#[wrapper(new(x: f32, y: f32))]
#[derive(Clone, Copy, Component, Default)]
pub struct Acceleration2D(pub Vec2);

#[wrapper]
#[derive(Clone, Copy, Component, Default)]
pub struct Rotation2D(pub f32);

#[wrapper]
#[derive(Clone, Copy, Component, Default)]
pub struct AngularVelocity2D(pub f32);

#[derive(Component)]
pub struct ShapeComponent(Box<dyn Shape2DExt>);

#[derive(Bundle)]
pub struct MovementBundle {
    position: Position2D,
    velocity: Velocity2D,
    acceleration: Acceleration2D,
    rotation: Rotation2D,
    angular_velocity: AngularVelocity2D,
}

impl MovementBundle {
    pub const INERT: Self = MovementBundle {
        position: Position2D(Vec2::ZERO),
        velocity: Velocity2D(Vec2::ZERO),
        acceleration: Acceleration2D(Vec2::ZERO),
        rotation: Rotation2D(0.0),
        angular_velocity: AngularVelocity2D(0.0),
    };
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::INERT
    }
}

impl ShapeComponent {
    pub fn new<S: Shape2DExt + 'static>(shape: S) -> Self {
        ShapeComponent(Box::new(shape))
    }
}

#[derive(Component)]
pub struct DrawScreen;
#[derive(Component)]
pub struct DrawWorld;

// i dont care
unsafe impl Sync for ShapeComponent {}
unsafe impl Send for ShapeComponent {}

gen_ref_type!(WorldImpl, Ecs, worlds);

impl Ecs {
    pub fn new() -> Self {
        WorldImpl::new().create()
    }
}

pub struct WorldImpl {
    inner: BWorld,
}

#[derive(ScheduleLabel, Clone, PartialEq, Eq, Debug, Hash)]
pub struct FrameEnd;

impl WorldImpl {
    fn new() -> Self {
        let mut world = BWorld::new();

        let mut frame_end = Schedule::new(FrameEnd);

        frame_end.add_systems(
            (
                acceleration,
                velocity,
                angular_velocity,
                (update_shape_positions, update_shape_rotations),
                draw_shapes_world,
                draw_shapes_screen,
            )
                .chain(),
        );

        world.add_schedule(frame_end);

        Self { inner: world }
    }

    fn update(&mut self) {
        self.inner.run_schedule(FrameEnd);
    }

    /// frame start
    pub fn add_systems<M>(&mut self, systems: impl IntoScheduleConfigs<ScheduleSystem, M>) {
        self.inner
            .resource_mut::<bevy_ecs::schedule::Schedules>()
            .entry(FrameEnd)
            .add_systems(systems);
    }
}

pub fn update() {
    for world in get_worlds_state() {
        world.update();
    }
}

pub fn init() {
    init_worlds_storage();
}

impl Deref for WorldImpl {
    type Target = BWorld;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for WorldImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

fn velocity(query: Query<(&mut Position2D, &Velocity2D)>) {
    for (mut pos, vel) in query {
        pos.0 += vel.0 * physics_delta_time();
    }
}

fn acceleration(query: Query<(&mut Velocity2D, &Acceleration2D)>) {
    for (mut vel, acc) in query {
        vel.0 += acc.0 * physics_delta_time();
    }
}

fn update_shape_positions(query: Query<(&mut ShapeComponent, &Position2D)>) {
    for (mut shape, pos) in query {
        shape.0.set_pos(pos.0);
    }
}

fn update_shape_rotations(query: Query<(&mut ShapeComponent, &Rotation2D)>) {
    for (mut shape, rot) in query {
        shape.0.set_rotation(rot.0);
    }
}

fn angular_velocity(query: Query<(&mut Rotation2D, &AngularVelocity2D)>) {
    for (mut rot, ang_vel) in query {
        rot.0 += ang_vel.0 * physics_delta_time();
    }
}

fn draw_shapes_screen(query: Query<(&ShapeComponent, &DrawScreen)>) {
    for (shape, _) in query {
        shape.0.draw();
    }
}

fn draw_shapes_world(query: Query<(&ShapeComponent, &DrawWorld)>) {
    for (shape, _) in query {
        shape.0.draw_world();
    }
}
