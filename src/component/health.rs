use bevy::prelude::*;
use std::ops::Deref;

#[cfg(feature = "egui")]
use bevy_inspector_egui::Inspectable;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "egui", derive(Inspectable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Component, Clone, Eq, PartialEq)]
pub struct Health {
    original_hp: u32,
    current_hp: u32,
    health_cap: Option<u32>,
}

impl Health {
    pub fn new(base_hp: u32) -> Self {
        Health {
            original_hp: base_hp,
            current_hp: base_hp,
            health_cap: None,
        }
    }

    pub fn take(&mut self, amount: u32) {
        self.current_hp = if let Some(new_hp) = self.current_hp.checked_sub(amount) {
            new_hp
        } else {
            0
        };
    }

    pub fn add(&mut self, amount: u32) {
        self.current_hp += amount;
    }

    pub fn reset(&mut self) {
        self.current_hp = self.original_hp;
    }

    pub fn is_zero(&self) -> bool {
        self.current_hp == 0
    }
}

impl Deref for Health {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.current_hp
    }
}
