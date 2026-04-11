use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use sge_vectors::Vec2;
use bvh::{
    aabb::{Aabb, Bounded},
    bounding_hierarchy::BHShape,
    bvh::Bvh,
};
use nalgebra::Point2;
use sge_macros::gen_ref_type;
use sge_math::collision::Aabb2d;
use sge_math::transform::Transform2D;
use sge_time::physics_delta_time;
use slotmap::{SlotMap, new_key_type};

gen_ref_type!(World, WorldRef, worlds);

new_key_type! {
    pub struct ColliderKey;
}

struct BvhEntry {
    key: ColliderKey,
    min: [f32; 2],
    max: [f32; 2],
    node_index: usize,
}

impl Bounded<f32, 2> for BvhEntry {
    fn aabb(&self) -> Aabb<f32, 2> {
        Aabb::with_bounds(
            Point2::new(self.min[0], self.min[1]).into(),
            Point2::new(self.max[0], self.max[1]).into(),
        )
    }
}

impl BHShape<f32, 2> for BvhEntry {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

pub struct World {
    id: usize,
    collisions_this_frame: HashMap<ColliderKey, Vec<(ColliderKey, CollisionPoints)>>,
    colliders: SlotMap<ColliderKey, Collider>,
    gravity: f32,
}

impl World {
    pub fn new() -> WorldRef {
        Self {
            id: get_worlds_state().len(),
            collisions_this_frame: HashMap::new(),
            colliders: SlotMap::with_key(),
            gravity: 9.81,
        }
        .create()
    }

    pub fn update(&mut self) {
        self.step_physics();
        self.rebuild_collisions();
        self.resolve_collisions();
    }

    fn step_physics(&mut self) {
        let dt = physics_delta_time();
        for (_, collider) in self.colliders.iter_mut() {
            collider.entity.step(dt, self.gravity);
        }
    }

    fn rebuild_collisions(&mut self) {
        self.collisions_this_frame.clear();

        let mut entries: Vec<BvhEntry> = self
            .colliders
            .iter()
            .map(|(key, collider)| {
                let aabb = collider.aabb();
                BvhEntry {
                    key,
                    min: [aabb.min.x, aabb.min.y],
                    max: [aabb.max.x, aabb.max.y],
                    node_index: 0,
                }
            })
            .collect();

        if entries.len() < 2 {
            return;
        }

        let bvh = Bvh::build(&mut entries);

        let mut candidates: Vec<(ColliderKey, ColliderKey)> = Vec::new();
        for entry in entries.iter() {
            let hits = bvh.traverse(&entry.aabb(), &entries);
            for hit in hits {
                if hit.key > entry.key {
                    candidates.push((entry.key, hit.key));
                }
            }
        }

        for (a, b) in candidates {
            let points = self.colliders[a].check_collision(&self.colliders[b]);
            if points.collision {
                self.collisions_this_frame
                    .entry(a)
                    .or_default()
                    .push((b, points));
                self.collisions_this_frame
                    .entry(b)
                    .or_default()
                    .push((a, invert(points)));
            }
        }
    }

    fn resolve_collisions(&mut self) {
        let pairs: Vec<(ColliderKey, ColliderKey, CollisionPoints)> = self
            .collisions_this_frame
            .iter()
            .flat_map(|(a, hits)| {
                hits.iter()
                    .filter(move |(b, _)| a < b)
                    .map(|(b, points)| (*a, *b, *points))
            })
            .collect();

        for (a, b, points) in pairs {
            let a_dynamic = self.colliders[a].entity.is_dynamic();
            let b_dynamic = self.colliders[b].entity.is_dynamic();

            match (a_dynamic, b_dynamic) {
                (false, false) => {}
                (true, false) => {
                    // normal points from b toward a, so push a in +normal direction
                    self.colliders[a]
                        .entity
                        .move_by(points.normal * points.depth);

                    let vel = self.colliders[a].entity.get_velocity();
                    let along = vel.dot(points.normal);
                    if along < 0.0 {
                        self.colliders[a]
                            .entity
                            .set_velocity(vel - points.normal * along);
                    }
                }
                (false, true) => {
                    // normal points from a toward b, so push b in -normal direction to escape a
                    self.colliders[b]
                        .entity
                        .move_by(-points.normal * points.depth);

                    let vel = self.colliders[b].entity.get_velocity();
                    let along = vel.dot(points.normal);
                    if along > 0.0 {
                        self.colliders[b]
                            .entity
                            .set_velocity(vel - points.normal * along);
                    }
                }
                (true, true) => {
                    let ma = self.colliders[a].entity.get_mass();
                    let mb = self.colliders[b].entity.get_mass();
                    let total = ma + mb;
                    let a_ratio = mb / total;
                    let b_ratio = ma / total;

                    self.colliders[a]
                        .entity
                        .move_by(points.normal * points.depth * a_ratio);
                    self.colliders[b]
                        .entity
                        .move_by(-points.normal * points.depth * b_ratio);

                    let va = self.colliders[a].entity.get_velocity();
                    let vb = self.colliders[b].entity.get_velocity();

                    let relative_along = (va - vb).dot(points.normal);
                    if relative_along < 0.0 {
                        let impulse = points.normal * relative_along;
                        self.colliders[a]
                            .entity
                            .set_velocity(va - impulse * a_ratio);
                        self.colliders[b]
                            .entity
                            .set_velocity(vb + impulse * b_ratio);
                    }
                }
            }
        }
    }

    fn insert_collider(&mut self, collider: Collider) -> ColliderKey {
        self.colliders.insert(collider)
    }

    pub fn create_dynamic(&mut self, bounds: Bounds) -> ColliderRef {
        let key = self.insert_collider(Collider {
            entity: Entity::new().dynamic(),
            bounds,
        });
        ColliderRef {
            world: WorldRef(self.id),
            key,
        }
    }

    pub fn create_fixed(&mut self, bounds: Bounds) -> ColliderRef {
        let key = self.insert_collider(Collider {
            entity: Entity::new().fixed(),
            bounds,
        });
        ColliderRef {
            world: WorldRef(self.id),
            key,
        }
    }

    pub fn remove(&mut self, key: ColliderKey) {
        self.colliders.remove(key);
        self.collisions_this_frame.remove(&key);
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
}

#[derive(Clone, Copy, Debug)]
pub struct Entity {
    velocity: Vec2,
    mass: f32,
    force: Vec2,
    acceleration: Vec2,
    jerk: Vec2,
    transform: Transform2D,
    ty: EntityMovementType,
    collision_type: EntityCollisionType,
}

#[derive(Clone, Copy, Debug)]
enum EntityCollisionType {
    Solid,
    Transparent,
}

#[derive(Clone, Copy, Debug)]
enum EntityMovementType {
    Dynamic,
    Fixed,
}

impl Entity {
    pub const fn new() -> Self {
        Self {
            velocity: Vec2::ZERO,
            mass: 1.0,
            force: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            jerk: Vec2::ZERO,
            transform: Transform2D::IDENTITY,
            ty: EntityMovementType::Dynamic,
            collision_type: EntityCollisionType::Solid,
        }
    }

    pub const fn dynamic(mut self) -> Self {
        self.ty = EntityMovementType::Dynamic;
        self
    }

    pub const fn fixed(mut self) -> Self {
        self.ty = EntityMovementType::Fixed;
        self
    }

    pub const fn transparent(mut self) -> Self {
        self.collision_type = EntityCollisionType::Transparent;
        self
    }

    pub const fn solid(mut self) -> Self {
        self.collision_type = EntityCollisionType::Solid;
        self
    }

    pub const fn is_transparent(&self) -> bool {
        matches!(self.collision_type, EntityCollisionType::Transparent)
    }

    pub const fn is_solid(&self) -> bool {
        matches!(self.collision_type, EntityCollisionType::Solid)
    }

    pub fn step(&mut self, dt: f32, gravity: f32) {
        if self.is_dynamic() {
            self.force += Vec2::new(0.0, self.mass * gravity);
            self.velocity += self.force / self.mass * dt;
            self.transform.translate_by(self.velocity * dt);
            self.force = Vec2::ZERO;
        }
    }

    pub const fn is_dynamic(&self) -> bool {
        matches!(self.ty, EntityMovementType::Dynamic)
    }

    pub const fn is_fixed(&self) -> bool {
        matches!(self.ty, EntityMovementType::Fixed)
    }

    pub const fn with_mass(mut self, mass: f32) -> Self {
        self.mass = mass;
        self
    }

    pub fn with_velocity(mut self, velocity: Vec2) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn with_position(mut self, position: Vec2) -> Self {
        self.transform.set_translation(position);
        self
    }

    pub fn with_acceleration(mut self, acceleration: Vec2) -> Self {
        self.acceleration = acceleration;
        self
    }

    pub fn with_jerk(mut self, jerk: Vec2) -> Self {
        self.jerk = jerk;
        self
    }

    pub fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }

    pub fn set_force(&mut self, force: Vec2) {
        self.force = force;
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.transform.set_translation(position);
    }

    pub fn set_acceleration(&mut self, acceleration: Vec2) {
        self.acceleration = acceleration;
    }

    pub fn set_jerk(&mut self, jerk: Vec2) {
        self.jerk = jerk;
    }

    pub fn add_velocity(&mut self, velocity: Vec2) {
        self.velocity += velocity;
    }

    pub fn add_force(&mut self, force: Vec2) {
        self.force += force;
    }

    pub fn add_acceleration(&mut self, acceleration: Vec2) {
        self.acceleration += acceleration;
    }

    pub fn add_jerk(&mut self, jerk: Vec2) {
        self.jerk += jerk;
    }

    pub fn move_by(&mut self, offset: Vec2) {
        self.transform.translate_by(offset);
    }

    pub fn move_to(&mut self, offset: Vec2) {
        self.transform.set_translation(offset);
    }

    pub fn get_velocity(&self) -> Vec2 {
        self.velocity
    }

    pub fn get_force(&self) -> Vec2 {
        self.force
    }

    pub fn get_position(&self) -> Vec2 {
        self.transform.translation()
    }

    pub fn get_acceleration(&self) -> Vec2 {
        self.acceleration
    }

    pub fn get_jerk(&self) -> Vec2 {
        self.jerk
    }

    pub fn get_mass(&self) -> f32 {
        self.mass
    }

    pub fn get_mass_mut(&mut self) -> &mut f32 {
        &mut self.mass
    }

    pub fn set_mass_mut(&mut self, mass: f32) {
        self.mass = mass;
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Bounds {
    Point,
    Rect(Vec2),
    Circle(f32),
}

#[derive(Clone, Copy, Debug)]
pub struct Collider {
    entity: Entity,
    bounds: Bounds,
}

#[derive(Clone, Copy, Debug)]
pub struct CollisionPoints {
    pub normal: Vec2,
    pub depth: f32,
    pub collision: bool,
}

fn invert(mut collision: CollisionPoints) -> CollisionPoints {
    collision.normal = -collision.normal;
    collision
}

fn circle_circle(c1: Vec2, r1: f32, c2: Vec2, r2: f32) -> CollisionPoints {
    let delta = c1 - c2;
    let dist = delta.length();
    let max_dist = r1 + r2;
    let collision = dist < max_dist;
    let depth = (max_dist - dist).max(0.0);
    let normal = if dist > 0.0 { delta / dist } else { Vec2::X };
    CollisionPoints {
        normal,
        depth,
        collision,
    }
}

fn circle_point(c: Vec2, r: f32, p: Vec2) -> CollisionPoints {
    let delta = c - p;
    let dist = delta.length();
    let collision = dist < r;
    let depth = (r - dist).max(0.0);
    let normal = if dist > 0.0 { delta / dist } else { Vec2::X };
    CollisionPoints {
        normal,
        depth,
        collision,
    }
}

fn point_point(p1: Vec2, p2: Vec2) -> CollisionPoints {
    let delta = p1 - p2;
    let dist = delta.length();
    let normal = if dist > 0.0 { delta / dist } else { Vec2::X };
    CollisionPoints {
        normal,
        depth: 0.0,
        collision: p1 == p2,
    }
}

fn rect_point(c: Vec2, size: Vec2, p: Vec2) -> CollisionPoints {
    let half = size * 0.5;
    let local = c - p;
    let inside = local.x.abs() < half.x && local.y.abs() < half.y;

    if !inside {
        return CollisionPoints {
            normal: Vec2::ZERO,
            depth: 0.0,
            collision: false,
        };
    }

    let overlap_x = half.x - local.x.abs();
    let overlap_y = half.y - local.y.abs();
    let (normal, depth) = if overlap_x < overlap_y {
        (Vec2::X * local.x.signum(), overlap_x)
    } else {
        (Vec2::Y * local.y.signum(), overlap_y)
    };

    CollisionPoints {
        normal,
        depth,
        collision: true,
    }
}

fn rect_rect(c1: Vec2, s1: Vec2, c2: Vec2, s2: Vec2) -> CollisionPoints {
    let half1 = s1 * 0.5;
    let half2 = s2 * 0.5;
    let delta = c2 - c1;

    let overlap_x = half1.x + half2.x - delta.x.abs();
    let overlap_y = half1.y + half2.y - delta.y.abs();

    if overlap_x <= 0.0 || overlap_y <= 0.0 {
        return CollisionPoints {
            normal: Vec2::ZERO,
            depth: 0.0,
            collision: false,
        };
    }

    let (normal, depth) = if overlap_x < overlap_y {
        (Vec2::X * delta.x.signum(), overlap_x)
    } else {
        (Vec2::Y * delta.y.signum(), overlap_y)
    };

    CollisionPoints {
        normal,
        depth,
        collision: true,
    }
}

fn rect_circle(c: Vec2, size: Vec2, circle: Vec2, radius: f32) -> CollisionPoints {
    let half = size * 0.5;
    let local = circle - c;
    let clamped = local.clamp(-half, half);
    let inside = clamped == local;

    if inside {
        let overlap_x = half.x - local.x.abs();
        let overlap_y = half.y - local.y.abs();
        let (normal, depth) = if overlap_x < overlap_y {
            (Vec2::X * local.x.signum(), overlap_x + radius)
        } else {
            (Vec2::Y * local.y.signum(), overlap_y + radius)
        };
        CollisionPoints {
            normal,
            depth,
            collision: true,
        }
    } else {
        let closest = c + clamped;
        let delta = circle - closest;
        let dist = delta.length();
        if dist >= radius {
            return CollisionPoints {
                normal: Vec2::ZERO,
                depth: 0.0,
                collision: false,
            };
        }
        let normal = if dist > 0.0 { delta / dist } else { Vec2::X };
        CollisionPoints {
            normal,
            depth: radius - dist,
            collision: true,
        }
    }
}

impl Collider {
    pub fn check_collision(&self, other: &Self) -> CollisionPoints {
        match (&self.bounds, &other.bounds) {
            (Bounds::Circle(r1), Bounds::Circle(r2)) => {
                circle_circle(self.get_position(), *r1, other.get_position(), *r2)
            }
            (Bounds::Point, Bounds::Point) => {
                point_point(self.get_position(), other.get_position())
            }
            (Bounds::Circle(r), Bounds::Point) => {
                circle_point(self.get_position(), *r, other.get_position())
            }
            (Bounds::Point, Bounds::Circle(r)) => {
                invert(circle_point(other.get_position(), *r, self.get_position()))
            }
            (Bounds::Point, Bounds::Rect(size)) => {
                invert(rect_point(other.get_position(), *size, self.get_position()))
            }
            (Bounds::Rect(size), Bounds::Point) => {
                rect_point(self.get_position(), *size, other.get_position())
            }
            (Bounds::Rect(size), Bounds::Circle(r)) => invert(rect_circle(
                self.get_position(),
                *size,
                other.get_position(),
                *r,
            )),
            (Bounds::Circle(r), Bounds::Rect(size)) => {
                rect_circle(other.get_position(), *size, self.get_position(), *r)
            }
            (Bounds::Rect(s1), Bounds::Rect(s2)) => invert(rect_rect(
                self.get_position(),
                *s1,
                other.get_position(),
                *s2,
            )),
        }
    }

    pub fn new(position: Vec2, bounds: Bounds) -> Self {
        Self {
            entity: Entity::new().with_position(position),
            bounds,
        }
    }

    pub fn aabb(&self) -> Aabb2d {
        let pos = self.entity.get_position();
        match &self.bounds {
            Bounds::Point => Aabb2d::new(pos, pos),
            Bounds::Rect(size) => {
                let half = size * 0.5;
                Aabb2d::new(pos - half, pos + half)
            }
            Bounds::Circle(r) => {
                let r = Vec2::splat(*r);
                Aabb2d::new(pos - r, pos + r)
            }
        }
    }
}

impl Deref for Collider {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}

impl DerefMut for Collider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}

pub struct ColliderRef {
    world: WorldRef,
    key: ColliderKey,
}

impl Deref for ColliderRef {
    type Target = Collider;

    fn deref(&self) -> &Self::Target {
        &self.world.colliders[self.key]
    }
}

impl DerefMut for ColliderRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.world.colliders[self.key]
    }
}

impl ColliderRef {
    pub fn collisions(&self) -> &[(ColliderKey, CollisionPoints)] {
        self.world
            .collisions_this_frame
            .get(&self.key)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn is_colliding(&self) -> bool {
        !self.collisions().is_empty()
    }

    pub fn is_colliding_with(&self, other: ColliderKey) -> bool {
        self.collisions().iter().any(|(key, _)| *key == other)
    }

    pub fn check_collision_with(&self, other: ColliderKey) -> Option<CollisionPoints> {
        self.collisions()
            .iter()
            .find(|(key, _)| *key == other)
            .map(|(_, points)| *points)
    }

    pub fn remove(mut self) {
        self.world.remove(self.key);
    }

    pub fn key(&self) -> ColliderKey {
        self.key
    }
}

pub fn init() {
    init_worlds_storage();
}
