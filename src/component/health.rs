//! Keeps track of an integer number of health
//!
//! Also supports a health cap and restoring to original health value.

use bevy::prelude::*;
use std::ops::Deref;

#[cfg(feature = "egui")]
use bevy_inspector_egui::Inspectable;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Health component, initilize only using then `new` function
#[cfg_attr(feature = "egui", derive(Inspectable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Component, Clone, Eq, PartialEq)]
pub struct Health {
    original_hp: u32,
    current_hp: u32,
    health_cap: Option<u32>,
}

impl Health {
    /// Initialize health with a base value
    pub fn new(base_hp: u32) -> Self {
        Health {
            original_hp: base_hp,
            current_hp: base_hp,
            health_cap: None,
        }
    }

    /// Lose health
    ///
    /// Health cannot go lower then zero. Attempting to take health away will result in no action.
    pub fn take(&mut self, amount: u32) {
        self.current_hp = if let Some(new_hp) = self.current_hp.checked_sub(amount) {
            new_hp
        } else {
            0
        };
    }

    /// Gain health
    ///
    /// If a health cap was specified, health will not exceed
    pub fn add(&mut self, amount: u32) {
        self.current_hp += amount;
    }

    /// Restore health to original value
    pub fn reset(&mut self) {
        self.current_hp = self.original_hp;
    }

    /// Query if health is zero
    pub fn is_zero(&self) -> bool {
        self.current_hp == 0
    }

    /// Get current health percentage based on max health
    // TODO watch for division by zero?
    pub fn percent(&self) -> f32 {
        self.current_hp as f32 / self.original_hp as f32
    }
}

impl Deref for Health {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.current_hp
    }
}
