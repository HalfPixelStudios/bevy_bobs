use bevy::{math::Mat2, prelude::*};
use std::f32::consts::PI;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub enum AttackPattern {
    Straight,
    Shotgun { shots: u32, angle: f32 },
    Around { shots: u32 },
}

pub fn generate_attack_points(dir: Vec2, attack_pattern: &AttackPattern) -> Vec<Vec2> {
    match attack_pattern {
        AttackPattern::Straight => return vec![dir],
        AttackPattern::Shotgun { shots, angle } => {
            let offset_start = (*shots as f32) * angle / 2.;
            return (0..*shots)
                .collect::<Vec<u32>>()
                .iter()
                .map(|i| Mat2::from_angle(-offset_start + (*i as f32) * angle) * dir)
                .collect();
        }
        AttackPattern::Around { shots } => {
            return (0..*shots)
                .collect::<Vec<u32>>()
                .iter()
                .map(|i| Mat2::from_angle((*i as f32) * 2. * PI / (*shots as f32)) * dir)
                .collect()
        }
    }
}
