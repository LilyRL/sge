use std::collections::HashMap;

use bevy_math::Vec2;
use player::{Player, PlayerKey};
use rapier2d::prelude::*;
use sge_api::shapes_2d::{
    draw_arrow, draw_arrow_world, draw_circle, draw_circle_outline, draw_circle_outline_world,
    draw_circle_world, draw_rect_outline, draw_rect_outline_world,
};
use sge_color::Color;
use sge_macros::gen_ref_type;
use sge_time::physics_delta_time;
use slotmap::{SlotMap, new_key_type};

pub mod player;

const PIXELS_PER_METRE: f32 = 100.0;

fn to_rapier(v: Vec2) -> Vector<Real> {
    vector![v.x / PIXELS_PER_METRE, -v.y / PIXELS_PER_METRE]
}

fn from_rapier(v: &Vector<Real>) -> Vec2 {
    Vec2::new(v.x * PIXELS_PER_METRE, -v.y * PIXELS_PER_METRE)
}

fn pos_to_rapier(v: Vec2) -> Isometry<Real> {
    Isometry::translation(v.x / PIXELS_PER_METRE, -v.y / PIXELS_PER_METRE)
}

fn pos_from_rapier(iso: &Isometry<Real>) -> Vec2 {
    Vec2::new(
        iso.translation.x * PIXELS_PER_METRE,
        -iso.translation.y * PIXELS_PER_METRE,
    )
}

gen_ref_type!(World, WorldRef, worlds);

new_key_type! {
    pub struct ObjectKey;
}

pub struct ObjectHandles {
    pub rigid_body: RigidBodyHandle,
    pub collider: rapier2d::geometry::ColliderHandle,
    pub bounds: Bounds,
    pub is_dynamic: bool,
}

pub struct World {
    id: usize,
    gravity: f32,

    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,

    objects: SlotMap<ObjectKey, ObjectHandles>,
    players: SlotMap<PlayerKey, Player>,
    handle_to_key: HashMap<RigidBodyHandle, ObjectKey>,

    collisions: HashMap<ObjectKey, Vec<CollisionInfo>>,
}

#[derive(Clone, Copy, Debug)]
pub struct CollisionInfo {
    pub other: ObjectRef,
    pub points: CollisionPoints,
    pub event: CollisionType,
}

#[derive(Clone, Copy, Debug)]
pub enum CollisionType {
    Started,
    Ongoing,
    Stopped,
}

impl CollisionType {
    pub fn is_colliding(&self) -> bool {
        matches!(self, Self::Started | Self::Ongoing)
    }
}

impl World {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> WorldRef {
        Self {
            id: get_worlds_state().len(),
            gravity: 980.0,

            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),

            objects: SlotMap::with_key(),
            players: SlotMap::with_key(),
            handle_to_key: HashMap::new(),
            collisions: HashMap::new(),
        }
        .create()
    }

    fn get_ref(&self) -> WorldRef {
        WorldRef(self.id)
    }

    pub fn update(&mut self) {
        let dt = physics_delta_time();

        let gravity = vector![0.0, -self.gravity / PIXELS_PER_METRE];

        self.update_players();

        let integration_parameters = IntegrationParameters {
            dt,
            ..Default::default()
        };

        let (collision_send, collision_recv) = std::sync::mpsc::channel();
        let (contact_force_send, _contact_force_recv) = std::sync::mpsc::channel();
        let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);

        self.physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &(),
            &event_handler,
        );

        self.rebuild_collisions(collision_recv);
    }

    fn rebuild_collisions(&mut self, collision_recv: std::sync::mpsc::Receiver<CollisionEvent>) {
        for infos in self.collisions.values_mut() {
            infos.retain(|i| !matches!(i.event, CollisionType::Stopped));
            for info in infos.iter_mut() {
                if matches!(info.event, CollisionType::Started) {
                    info.event = CollisionType::Ongoing;
                }
            }
        }
        self.collisions.retain(|_, v| !v.is_empty());

        let world_ref = WorldRef(self.id);

        while let Ok(event) = collision_recv.try_recv() {
            match event {
                CollisionEvent::Started(ch1, ch2, _) => {
                    let rb1 = self.collider_set[ch1].parent();
                    let rb2 = self.collider_set[ch2].parent();
                    let (rb1, rb2) = match (rb1, rb2) {
                        (Some(a), Some(b)) => (a, b),
                        _ => continue,
                    };
                    let key1 = match self.handle_to_key.get(&rb1) {
                        Some(k) => *k,
                        None => continue,
                    };
                    let key2 = match self.handle_to_key.get(&rb2) {
                        Some(k) => *k,
                        None => continue,
                    };
                    let points = self.compute_collision_points(ch1, ch2, key1, key2);
                    self.collisions
                        .entry(key1)
                        .or_default()
                        .push(CollisionInfo {
                            other: ObjectRef {
                                world: world_ref,
                                key: key2,
                            },
                            points,
                            event: CollisionType::Started,
                        });
                    self.collisions
                        .entry(key2)
                        .or_default()
                        .push(CollisionInfo {
                            other: ObjectRef {
                                world: world_ref,
                                key: key1,
                            },
                            points: invert(points),
                            event: CollisionType::Started,
                        });
                }
                CollisionEvent::Stopped(ch1, ch2, _) => {
                    let rb1 = self.collider_set[ch1].parent();
                    let rb2 = self.collider_set[ch2].parent();
                    let (rb1, rb2) = match (rb1, rb2) {
                        (Some(a), Some(b)) => (a, b),
                        _ => continue,
                    };
                    let key1 = match self.handle_to_key.get(&rb1) {
                        Some(k) => *k,
                        None => continue,
                    };
                    let key2 = match self.handle_to_key.get(&rb2) {
                        Some(k) => *k,
                        None => continue,
                    };
                    if let Some(infos) = self.collisions.get_mut(&key1)
                        && let Some(info) = infos.iter_mut().find(|i| i.other.key == key2)
                    {
                        info.event = CollisionType::Stopped;
                    }
                    if let Some(infos) = self.collisions.get_mut(&key2)
                        && let Some(info) = infos.iter_mut().find(|i| i.other.key == key1)
                    {
                        info.event = CollisionType::Stopped;
                    }
                }
            }
        }
    }

    fn compute_collision_points(
        &self,
        ch1: rapier2d::geometry::ColliderHandle,
        ch2: rapier2d::geometry::ColliderHandle,
        key1: ObjectKey,
        key2: ObjectKey,
    ) -> CollisionPoints {
        if let Some(pair) = self.narrow_phase.contact_pair(ch1, ch2) {
            for manifold in &pair.manifolds {
                let normal_rapier = manifold.data.normal;
                let normal = Vec2::new(normal_rapier.x, -normal_rapier.y);

                let depth = manifold
                    .points
                    .iter()
                    .filter(|p| p.dist < 0.0)
                    .map(|p| -p.dist * PIXELS_PER_METRE)
                    .fold(0.0_f32, f32::max);

                if depth > 0.0 {
                    return CollisionPoints {
                        normal,
                        depth,
                        collision: true,
                    };
                }
            }
        }

        let pos1 = self.get_position_by_key(key1);
        let pos2 = self.get_position_by_key(key2);
        let delta = pos1 - pos2;
        let dist = delta.length();
        let normal = if dist > 0.0 { delta / dist } else { Vec2::X };
        CollisionPoints {
            normal,
            depth: 0.0,
            collision: true,
        }
    }

    fn get_position_by_key(&self, key: ObjectKey) -> Vec2 {
        let rb_handle = self.objects[key].rigid_body;
        pos_from_rapier(self.rigid_body_set[rb_handle].position())
    }

    fn insert_object(&mut self, bounds: Bounds, rigid_body: RigidBody) -> ObjectKey {
        let is_dynamic = rigid_body.is_dynamic();
        let rb_handle = self.rigid_body_set.insert(rigid_body);

        let rapier_collider = bounds_to_rapier_collider(&bounds);
        let col_handle = self.collider_set.insert_with_parent(
            rapier_collider,
            rb_handle,
            &mut self.rigid_body_set,
        );

        let key = self.objects.insert(ObjectHandles {
            rigid_body: rb_handle,
            collider: col_handle,
            bounds,
            is_dynamic,
        });
        self.handle_to_key.insert(rb_handle, key);
        key
    }

    pub fn create_dynamic(&mut self, bounds: Bounds) -> ObjectRef {
        let rb = RigidBodyBuilder::dynamic().build();
        let key = self.insert_object(bounds, rb);
        ObjectRef {
            world: WorldRef(self.id),
            key,
        }
    }

    pub fn create_fixed(&mut self, bounds: Bounds) -> ObjectRef {
        let rb = RigidBodyBuilder::fixed().build();
        let key = self.insert_object(bounds, rb);
        ObjectRef {
            world: WorldRef(self.id),
            key,
        }
    }

    pub fn create_kinematic(&mut self, bounds: Bounds) -> crate::ObjectRef {
        use rapier2d::prelude::RigidBodyBuilder;
        let rb = RigidBodyBuilder::kinematic_position_based().build();
        let key = self.insert_object(bounds, rb);
        crate::ObjectRef {
            world: crate::WorldRef(self.id),
            key,
        }
    }

    pub fn remove(&mut self, key: ObjectKey) {
        if let Some(handles) = self.objects.remove(key) {
            self.handle_to_key.remove(&handles.rigid_body);
            self.collider_set.remove(
                handles.collider,
                &mut self.island_manager,
                &mut self.rigid_body_set,
                true,
            );
            self.rigid_body_set.remove(
                handles.rigid_body,
                &mut self.island_manager,
                &mut self.collider_set,
                &mut self.impulse_joint_set,
                &mut self.multibody_joint_set,
                true,
            );
        }
        self.collisions.remove(&key);
    }

    pub fn set_gravity(&mut self, gravity: f32) {
        self.gravity = gravity;
    }

    pub fn get_gravity(&self) -> f32 {
        self.gravity
    }

    pub fn get_gravity_mut(&mut self) -> &mut f32 {
        &mut self.gravity
    }

    fn get_position(&self, key: ObjectKey) -> Vec2 {
        self.get_position_by_key(key)
    }

    fn set_position(&mut self, key: ObjectKey, pos: Vec2) {
        let rb_handle = self.objects[key].rigid_body;
        let rb = &mut self.rigid_body_set[rb_handle];
        rb.set_position(pos_to_rapier(pos), true);
    }

    fn get_velocity(&self, key: ObjectKey) -> Vec2 {
        let rb_handle = self.objects[key].rigid_body;
        from_rapier(self.rigid_body_set[rb_handle].linvel())
    }

    fn set_velocity(&mut self, key: ObjectKey, vel: Vec2) {
        let rb_handle = self.objects[key].rigid_body;
        self.rigid_body_set[rb_handle].set_linvel(to_rapier(vel), true);
    }

    fn add_velocity(&mut self, key: ObjectKey, vel: Vec2) {
        let current = self.get_velocity(key);
        self.set_velocity(key, current + vel);
    }

    fn add_force(&mut self, key: ObjectKey, force: Vec2) {
        let rb_handle = self.objects[key].rigid_body;
        self.rigid_body_set[rb_handle].add_force(to_rapier(force), true);
    }

    fn get_mass(&self, key: ObjectKey) -> f32 {
        let rb_handle = self.objects[key].rigid_body;
        self.rigid_body_set[rb_handle].mass()
    }

    fn move_by(&mut self, key: ObjectKey, offset: Vec2) {
        let current = self.get_position(key);
        self.set_position(key, current + offset);
    }

    fn get_bounds(&self, key: ObjectKey) -> Bounds {
        self.objects[key].bounds
    }

    fn is_dynamic(&self, key: ObjectKey) -> bool {
        self.objects[key].is_dynamic
    }

    pub fn draw_colliders(&self) {
        for handles in self.objects.values() {
            let pos = pos_from_rapier(self.rigid_body_set[handles.rigid_body].position());
            match handles.bounds {
                Bounds::Circle(r) => draw_circle_outline(pos, r, Color::RED_500, 1.5),
                Bounds::Rect(size) => {
                    draw_rect_outline(pos - size * 0.5, size, 1.0, Color::RED_500)
                }
                Bounds::Point => draw_circle(pos, 2.0, Color::RED_500),
            }
        }

        for (a, infos) in &self.collisions {
            let a = &self.objects[*a];
            let pos = pos_from_rapier(self.rigid_body_set[a.rigid_body].position());

            for info in infos {
                let display_length = if info.points.depth > 0.0 {
                    info.points.depth
                } else {
                    60.0
                };
                draw_arrow(
                    pos,
                    pos + info.points.normal * display_length,
                    2.0,
                    Color::NEUTRAL_500.with_alpha(0.3),
                );
            }

            if !infos.is_empty() {
                match a.bounds {
                    Bounds::Circle(r) => draw_circle_outline(pos, r, Color::YELLOW_500, 3.0),
                    Bounds::Rect(size) => {
                        draw_rect_outline(pos - size * 0.5, size, 2.0, Color::YELLOW_500)
                    }
                    Bounds::Point => draw_circle(pos, 2.0, Color::YELLOW_500),
                }
            }
        }
    }

    pub fn draw_colliders_world(&self) {
        for handles in self.objects.values() {
            let pos = pos_from_rapier(self.rigid_body_set[handles.rigid_body].position());
            match handles.bounds {
                Bounds::Circle(r) => draw_circle_outline_world(pos, r, Color::RED_500, 1.5),
                Bounds::Rect(size) => {
                    draw_rect_outline_world(pos - size * 0.5, size, 1.0, Color::RED_500)
                }
                Bounds::Point => draw_circle_world(pos, 2.0, Color::RED_500),
            }
        }

        for (a, infos) in &self.collisions {
            let a = &self.objects[*a];
            let pos = pos_from_rapier(self.rigid_body_set[a.rigid_body].position());

            for info in infos {
                let display_length = if info.points.depth > 0.0 {
                    info.points.depth
                } else {
                    60.0
                };
                draw_arrow_world(
                    pos,
                    pos + info.points.normal * display_length,
                    2.0,
                    Color::NEUTRAL_500.with_alpha(0.3),
                );
            }

            if !infos.is_empty() {
                match a.bounds {
                    Bounds::Circle(r) => draw_circle_outline_world(pos, r, Color::YELLOW_500, 3.0),
                    Bounds::Rect(size) => {
                        draw_rect_outline_world(pos - size * 0.5, size, 2.0, Color::YELLOW_500)
                    }
                    Bounds::Point => draw_circle_world(pos, 2.0, Color::YELLOW_500),
                }
            }
        }
    }
}

fn bounds_to_rapier_collider(bounds: &Bounds) -> rapier2d::geometry::Collider {
    match bounds {
        Bounds::Circle(r) => ColliderBuilder::ball(r / PIXELS_PER_METRE)
            .restitution(0.0)
            .friction(0.5)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build(),
        Bounds::Rect(size) => ColliderBuilder::cuboid(
            size.x * 0.5 / PIXELS_PER_METRE,
            size.y * 0.5 / PIXELS_PER_METRE,
        )
        .restitution(0.0)
        .friction(0.5)
        .active_events(ActiveEvents::COLLISION_EVENTS)
        .build(),
        Bounds::Point => ColliderBuilder::ball(0.001 / PIXELS_PER_METRE)
            .restitution(0.0)
            .friction(0.5)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build(),
    }
}

fn invert(mut c: CollisionPoints) -> CollisionPoints {
    c.normal = -c.normal;
    c
}

#[derive(Clone, Copy, Debug)]
pub enum Bounds {
    Point,
    Rect(Vec2),
    Circle(f32),
}

#[derive(Clone, Copy, Debug)]
pub struct CollisionPoints {
    pub normal: Vec2,
    pub depth: f32,
    pub collision: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ObjectRef {
    world: WorldRef,
    key: ObjectKey,
}

impl ObjectRef {
    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.set_position(pos);
        self
    }

    pub fn get_position(&self) -> Vec2 {
        self.world.get_position(self.key)
    }

    pub fn set_position(&mut self, pos: Vec2) {
        self.world.set_position(self.key, pos);
    }

    pub fn get_velocity(&self) -> Vec2 {
        self.world.get_velocity(self.key)
    }

    pub fn set_velocity(&mut self, vel: Vec2) {
        self.world.set_velocity(self.key, vel);
    }

    pub fn add_velocity(&mut self, vel: Vec2) {
        self.world.add_velocity(self.key, vel);
    }

    pub fn add_force(&mut self, force: Vec2) {
        self.world.add_force(self.key, force);
    }

    pub fn get_mass(&self) -> f32 {
        self.world.get_mass(self.key)
    }

    pub fn move_by(&mut self, offset: Vec2) {
        self.world.move_by(self.key, offset);
    }

    pub fn get_bounds(&self) -> Bounds {
        self.world.get_bounds(self.key)
    }

    pub fn is_dynamic(&self) -> bool {
        self.world.is_dynamic(self.key)
    }

    pub fn collisions(&self) -> &[CollisionInfo] {
        self.world
            .collisions
            .get(&self.key)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn is_colliding(&self) -> bool {
        !self.collisions().is_empty()
    }

    pub fn is_colliding_with(&self, other: ObjectRef) -> bool {
        self.collisions().iter().any(|i| i.other == other)
    }

    pub fn check_collision_with(&self, other: ObjectRef) -> Option<CollisionPoints> {
        self.collisions()
            .iter()
            .find(|i| i.other == other)
            .map(|i| i.points)
    }

    pub fn remove(mut self) {
        self.world.remove(self.key);
    }

    pub fn key(&self) -> ObjectKey {
        self.key
    }

    pub fn rigidbody(&mut self) -> &mut RigidBody {
        let rb_handle = self.world.objects[self.key].rigid_body;
        &mut self.world.rigid_body_set[rb_handle]
    }

    pub fn with_ccd(mut self) -> Self {
        let rb_handle = self.world.objects[self.key].rigid_body;
        self.world.rigid_body_set[rb_handle].enable_ccd(true);
        self
    }
}

pub fn init() {
    init_worlds_storage();
}
