use std::{
    f32::consts::FRAC_PI_4,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

use bevy_math::Vec2;
use nalgebra::{Vector2, vector};
use rapier2d::{control::KinematicCharacterController, math::Isometry, prelude::QueryFilter};
use sge_input::{Button, KeyCode, button_held, button_pressed};
use sge_time::{physics_delta_time, time};
use slotmap::new_key_type;

new_key_type! {
    pub struct PlayerKey;
}

use crate::*;

pub(crate) struct Player {
    key: PlayerKey,
    object: ObjectKey,
    kcc: KinematicCharacterController,
    floor_max_angle: f32,
    binds: PlayerBinds,
    coyote_time: f32,
    time_last_on_ground: f32,
    velocity: Vec2,
    delta_translation: Vec2,
    delta_translation_last_frame: Vec2,
    position_last_frame: Vec2,
    is_sliding_down_slope: bool,

    was_on_ground: bool,
    is_really_on_ground: bool,

    num_double_jumps: usize,
    double_jumps_left: usize,

    move_speed: f32,
    jump_velocity: f32,
    double_jump_velocity: Option<f32>,

    just_jumped: bool,
    just_double_jumped: bool,
}

impl Player {
    fn double_jump_velocity(&self) -> f32 {
        self.double_jump_velocity.unwrap_or(self.jump_velocity)
    }
}

#[derive(Clone, Copy)]
struct PlayerRef {
    key: PlayerKey,
    world: WorldRef,
}

impl PlayerRef {
    pub fn object(&self) -> &ObjectHandles {
        &self.world.objects[self.object]
    }

    pub fn object_mut(&mut self) -> &mut ObjectHandles {
        let key = self.object;
        &mut self.world.objects[key]
    }
}

impl Deref for PlayerRef {
    type Target = Player;

    fn deref(&self) -> &Self::Target {
        &self.world.players[self.key]
    }
}

impl DerefMut for PlayerRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.world.players[self.key]
    }
}

pub struct PlayerController {
    value: PlayerRef,
}

impl World {
    pub fn create_player_controller(&mut self, bounds: Bounds) -> PlayerController {
        let object = self.create_kinematic(bounds).key;

        let player = Player {
            #[allow(clippy::uninit_assumed_init)]
            #[allow(invalid_value)]
            key: unsafe { MaybeUninit::uninit().assume_init() },
            kcc: KinematicCharacterController::default(),
            floor_max_angle: FRAC_PI_4,
            binds: PlayerBinds::default(),
            num_double_jumps: 1,
            double_jumps_left: 1,
            coyote_time: 0.1,
            time_last_on_ground: f32::NEG_INFINITY,
            is_really_on_ground: false,
            was_on_ground: false,
            velocity: Vec2::ZERO,
            object,
            double_jump_velocity: None,
            jump_velocity: 500.0,
            move_speed: 200.0,
            delta_translation: Vec2::ZERO,
            delta_translation_last_frame: Vec2::ZERO,
            is_sliding_down_slope: false,
            just_jumped: false,
            just_double_jumped: false,
            position_last_frame: Vec2::ZERO,
        };

        let key = self.players.insert(player);
        self.players[key].key = key;

        PlayerController {
            value: PlayerRef {
                key,
                world: self.get_ref(),
            },
        }
    }

    pub(crate) fn update_players(&mut self) {
        let dt = physics_delta_time();
        let now = time();

        let keys: Vec<PlayerKey> = self.players.keys().collect();

        for pk in keys {
            self.step_player(pk, dt, now);
            self.populate_player_collisions(pk);
        }
    }

    fn populate_player_collisions(&mut self, pk: PlayerKey) {
        let p = &self.players[pk];
        let player_key = p.object;
        let collider_handle = self.objects[player_key].collider;
        let world_ref = WorldRef(self.id);

        let contacts: Vec<(ObjectKey, CollisionPoints)> = self
            .narrow_phase
            .contact_pairs_with(collider_handle)
            .filter_map(|contact_pair| {
                let other_collider = if contact_pair.collider1 == collider_handle {
                    contact_pair.collider2
                } else {
                    contact_pair.collider1
                };
                let rb = self.collider_set[other_collider].parent()?;
                let other_key = *self.handle_to_key.get(&rb)?;
                let points = self.compute_collision_points(
                    collider_handle,
                    other_collider,
                    player_key,
                    other_key,
                );
                Some((other_key, points))
            })
            .collect();

        if let Some(old) = self.collisions.get(&player_key) {
            let old_keys: Vec<ObjectKey> = old.iter().map(|i| i.other.key).collect();
            for old_key in old_keys {
                if let Some(infos) = self.collisions.get_mut(&old_key) {
                    infos.retain(|i| i.other.key != player_key);
                }
            }
        }

        self.collisions.remove(&player_key);

        for (other_key, points) in contacts {
            self.collisions
                .entry(player_key)
                .or_default()
                .push(CollisionInfo {
                    other: ObjectRef {
                        world: world_ref,
                        key: other_key,
                    },
                    points,
                    event: CollisionType::Ongoing,
                });
            self.collisions
                .entry(other_key)
                .or_default()
                .push(CollisionInfo {
                    other: ObjectRef {
                        world: world_ref,
                        key: player_key,
                    },
                    points: invert(points),
                    event: CollisionType::Ongoing,
                });
        }
    }

    fn step_player(&mut self, pk: PlayerKey, dt: f32, now: f32) {
        let p = &mut self.players[pk];
        let binds = p.binds;

        let jump = button_pressed(binds.jump);
        let left = button_held(binds.left);
        let right = button_held(binds.right);

        p.velocity.y -= self.gravity * dt;
        p.velocity.x = (right as i32 - left as i32) as f32 * p.move_speed;

        p.just_double_jumped = false;
        p.just_jumped = false;

        let in_coyote = (now - p.time_last_on_ground) <= p.coyote_time;
        let can_jump = p.is_really_on_ground || in_coyote;

        if jump {
            if can_jump {
                p.velocity.y = p.jump_velocity;
                p.just_jumped = true;
                p.time_last_on_ground = f32::NEG_INFINITY;
                p.double_jumps_left = p.num_double_jumps;
            } else if p.double_jumps_left > 0 {
                p.velocity.y = p.double_jump_velocity();
                p.double_jumps_left -= 1;
                p.just_double_jumped = true;
            }
        }

        let desired_translation: Vector2<f32> = (p.velocity * dt).into();

        let rb_handle = self.objects[p.object].rigid_body;
        let current_pos = *self.rigid_body_set[rb_handle].position();
        let collider_handle = self.objects[p.object].collider;
        let collider_shape = self.collider_set[collider_handle].shape();
        let query_pipeline = &self.broad_phase.as_query_pipeline(
            self.narrow_phase.query_dispatcher(),
            &self.rigid_body_set,
            &self.collider_set,
            QueryFilter::default().exclude_rigid_body(rb_handle),
        );

        let movement = p.kcc.move_shape(
            dt,
            query_pipeline,
            collider_shape,
            &current_pos,
            desired_translation,
            |_| {},
        );

        p.is_sliding_down_slope = movement.is_sliding_down_slope;

        let new_pos = Isometry::translation(
            current_pos.translation.x + movement.translation.x,
            current_pos.translation.y + movement.translation.y,
        );
        self.rigid_body_set[rb_handle].set_next_kinematic_position(new_pos);

        let delta_translation_last_frame = p.delta_translation;
        let new_pos = crate::from_rapier(&vector![new_pos.translation.x, new_pos.translation.y]);
        p.delta_translation = new_pos - p.position_last_frame;
        p.delta_translation_last_frame = delta_translation_last_frame;
        p.position_last_frame = new_pos;

        let was_on_ground = p.is_really_on_ground;
        let now_grounded = movement.grounded;
        p.is_really_on_ground = now_grounded;
        p.was_on_ground = was_on_ground;

        let just_landed = now_grounded && !was_on_ground;

        if just_landed {
            p.double_jumps_left = p.num_double_jumps;
        }

        if now_grounded {
            p.time_last_on_ground = now;
            p.velocity.y = 0.0;
        }
    }
}

impl PlayerController {
    pub fn set_binds(&mut self) -> PlayerBindBuilder {
        PlayerBindBuilder { value: self.value }
    }

    pub fn velocity(&self) -> Vec2 {
        self.value.velocity
    }

    pub fn object(&self) -> &ObjectHandles {
        self.value.object()
    }

    pub fn object_mut(&mut self) -> &mut ObjectHandles {
        self.value.object_mut()
    }

    pub fn set_double_jumps(&mut self, num: usize) {
        self.value.num_double_jumps = num;
    }

    pub fn get_double_jumps(&self) -> usize {
        self.value.num_double_jumps
    }

    pub fn is_in_coyote(&self) -> bool {
        let now = time();
        (now - self.value.time_last_on_ground) <= self.value.coyote_time
    }

    pub fn is_on_ground(&self) -> bool {
        self.value.is_really_on_ground || self.is_in_coyote()
    }

    pub fn is_really_on_ground(&self) -> bool {
        self.value.is_really_on_ground
    }

    pub fn set_coyote_time(&mut self, ms: f32) {
        self.value.coyote_time = ms / 1000.0;
    }

    pub fn get_coyote_time(&self) -> f32 {
        self.value.coyote_time * 1000.0
    }

    pub fn can_jump(&self) -> bool {
        self.is_on_ground() || self.value.double_jumps_left > 0
    }

    pub fn set_max_floor_angle(&mut self, angle: f32) {
        self.value.floor_max_angle = angle;
    }

    pub fn set_position(&mut self, pos: Vec2) {
        let rb_handle = self.value.object().rigid_body;
        let iso = Isometry::translation(pos.x, pos.y);
        self.value.world.rigid_body_set[rb_handle].set_next_kinematic_position(iso);
    }

    pub fn position(&self) -> Vec2 {
        let rb_handle = self.value.object().rigid_body;
        let iso = self.value.world.rigid_body_set[rb_handle].position();
        crate::pos_from_rapier(iso)
    }

    pub fn time_last_on_ground(&self) -> f32 {
        self.value.time_last_on_ground
    }

    pub fn set_velocity(&mut self, vel: Vec2) {
        self.value.velocity = vel;
    }

    pub fn add_impulse(&mut self, impulse: Vec2) {
        self.value.velocity += impulse;
    }

    pub fn delta_translation(&self) -> Vec2 {
        self.value.delta_translation
    }

    pub fn position_last_frame(&self) -> Vec2 {
        self.position() - self.delta_translation()
    }

    pub fn is_sliding_down_slope(&self) -> bool {
        self.value.is_sliding_down_slope
    }

    pub fn was_on_ground(&self) -> bool {
        self.value.was_on_ground
    }

    pub fn just_landed(&self) -> bool {
        self.is_on_ground() && !self.value.was_on_ground
    }

    pub fn double_jumps_left(&self) -> usize {
        self.value.double_jumps_left
    }

    pub fn move_speed(&self) -> f32 {
        self.value.move_speed
    }

    pub fn set_move_speed(&mut self, speed: f32) {
        self.value.move_speed = speed;
    }

    pub fn jump_velocity(&self) -> f32 {
        self.value.jump_velocity
    }

    pub fn set_jump_velocity(&mut self, velocity: f32) {
        self.value.jump_velocity = velocity;
    }

    pub fn double_jump_velocity(&self) -> Option<f32> {
        self.value.double_jump_velocity
    }

    pub fn set_double_jump_velocity(&mut self, velocity: f32) {
        self.value.double_jump_velocity = Some(velocity);
    }

    pub fn set_double_jump_velocity_none(&mut self) {
        self.value.double_jump_velocity = None;
    }

    pub fn just_jumped(&self) -> bool {
        self.value.just_jumped
    }

    pub fn just_double_jumped(&self) -> bool {
        self.value.just_double_jumped
    }

    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.set_position(pos);
        self
    }

    pub fn is_moving(&self) -> bool {
        self.value.velocity != Vec2::ZERO
    }
}

#[derive(Clone, Copy)]
struct PlayerBinds {
    left: Button,
    right: Button,
    jump: Button,
}

impl Default for PlayerBinds {
    fn default() -> Self {
        PlayerBinds {
            left: Button::Keyboard(KeyCode::KeyA),
            right: Button::Keyboard(KeyCode::KeyD),
            jump: Button::Keyboard(KeyCode::Space),
        }
    }
}

pub struct PlayerBindBuilder {
    value: PlayerRef,
}

impl PlayerBindBuilder {
    pub fn left(mut self, button: impl Into<Button>) -> Self {
        self.value.binds.left = button.into();
        self
    }

    pub fn right(mut self, button: impl Into<Button>) -> Self {
        self.value.binds.right = button.into();
        self
    }

    pub fn jump(mut self, button: impl Into<Button>) -> Self {
        self.value.binds.jump = button.into();
        self
    }
}
