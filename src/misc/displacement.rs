//! Keeps track of the distance and displacement for an entity

use bevy::prelude::*;

pub struct Displacement {
    prev_pos: Option<Vec3>,
    total_distance: f32,
}

impl Displacement {
    pub fn new() -> Self {
        Displacement {
            prev_pos: None,
            total_distance: 0.,
        }
    }

    /// Fetch displacement since last tick
    pub fn get_displacement(&self, new_pos: Vec3) -> f32 {
        match self.prev_pos {
            Some(prev_pos) => prev_pos.distance(new_pos),
            None => 0.,
        }
    }

    /// Fetch total distance travelled
    pub fn get_distance(&self) -> f32 {
        self.total_distance
    }

    /// Update displacement values
    pub fn update(&mut self, new_pos: Vec3) {
        if let Some(prev_pos) = self.prev_pos {
            self.total_distance += prev_pos.distance(new_pos);
        }
        self.prev_pos = Some(new_pos);
    }

    /// Restore to original state
    pub fn reset(&mut self) {
        self.total_distance = 0.;
    }
}
