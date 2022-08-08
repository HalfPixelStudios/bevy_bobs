//! Simple 2d physics
//!
//! Apply forces to objects, apply damping and gravity.

use bevy::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Resource to influence the gravity of all the entities in the world
pub struct WorldGravity(pub Vec2);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Component, Clone, Default)]
pub struct RigidBody {
    pub mass: f32,
    /// override the world's gravity
    pub gravity_scale: Option<Vec2>,
    pub linear_damping: f32,
    pub velocity: Vec2,
    pub max_velocity: Option<f32>,
    pub force: Vec2,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldGravity(Vec2::ZERO))
            .add_system(velocity_system)
            .add_system(apply_force_system);
    }
}

/// System to resolve forces and update velocities
pub fn apply_force_system(world_gravity: Res<WorldGravity>, mut query: Query<&mut RigidBody>) {
    for mut rb in query.iter_mut() {
        let force = rb.force;
        let mass = rb.mass;
        rb.velocity += force / mass;

        // clamp max velocity
        let max_velocity = rb.max_velocity;
        if let Some(max_velocity) = max_velocity {
            rb.velocity = rb.velocity.clamp_length_max(max_velocity);
        }

        // reset force
        rb.force = if let Some(grav_override) = rb.gravity_scale {
            grav_override
        } else {
            world_gravity.0
        }
    }
}

/// Modifies translation based on velocity
pub fn velocity_system(time: Res<Time>, mut query: Query<(&mut Transform, &RigidBody)>) {
    for (mut trans, rb) in query.iter_mut() {
        trans.translation += rb.velocity.extend(0.) * time.delta_seconds();
    }
}
