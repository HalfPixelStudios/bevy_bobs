//! Simple drop table component
//!
//! Add drops and then randomly select them

use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng, Rng};

/// Droptable component
#[derive(Component)]
pub struct DropTable<T> {
    drops: Vec<(T, f32)>,
}

impl<T> DropTable<T> {
    /// Construct a new droptable from a vector of drops
    pub fn new(drops: Vec<(T, f32)>) -> Self {
        DropTable { drops }
    }

    /// Randomly select multiple drops
    pub fn select_drops(&self) -> Vec<&T> {
        let mut rng = thread_rng();
        let mut drops = vec![];
        for (id, prob) in self.drops.iter() {
            let rand_num = rng.gen::<f32>();
            if rand_num < *prob {
                drops.push(id);
            }
        }
        drops
    }

    /// Randomly select a single drop
    pub fn select_single_drop(&self) -> Option<&T> {
        if self.drops.is_empty() {
            return None;
        }

        self.drops.choose(&mut thread_rng()).map(|(id, _prob)| id)
    }
}
