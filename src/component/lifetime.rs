use bevy::prelude::*;
use std::time::Duration;

#[cfg(feature = "egui")]
use bevy_inspector_egui::{egui::Ui, Context, Inspectable};

use super::health::Health;
use crate::misc::displacement::*;

pub trait Lifetime {
    fn is_expired(&self) -> bool;
    fn reset(&mut self);
}

#[derive(Component)]
pub struct DistanceLifetime {
    max_distance: f32,
    displacement: Displacement,
}

#[derive(Component)]
pub struct DurationLifetime {
    timer: Timer,
}

#[derive(Component)]
pub struct PenetrationLifetime {
    health: Health,
}

impl DistanceLifetime {
    pub fn new(max_distance: f32) -> Self {
        DistanceLifetime {
            max_distance,
            displacement: Displacement::new(),
        }
    }
}

impl Lifetime for DistanceLifetime {
    fn is_expired(&self) -> bool {
        self.displacement.get_distance() > self.max_distance
    }
    fn reset(&mut self) {
        self.displacement.reset();
    }
}

// TODO unfortunately we cannot write a blanket implementation for all structs that implement Lifetime (due to E0210)
#[cfg(feature = "egui")]
impl Inspectable for DistanceLifetime {
    type Attributes = ();

    fn ui(&mut self, _ui: &mut Ui, _options: Self::Attributes, _context: &mut Context) -> bool {
        println!("{}", self.is_expired());
        false
    }
}

impl DurationLifetime {
    pub fn new(max_duration: f32) -> Self {
        DurationLifetime {
            timer: Timer::new(Duration::from_millis((max_duration * 1000.) as u64), false),
        }
    }
}

impl Lifetime for DurationLifetime {
    fn is_expired(&self) -> bool {
        self.timer.finished()
    }
    fn reset(&mut self) {
        self.timer.reset();
    }
}

#[cfg(feature = "egui")]
impl Inspectable for DurationLifetime {
    type Attributes = ();

    fn ui(&mut self, _ui: &mut Ui, _options: Self::Attributes, _context: &mut Context) -> bool {
        println!("{}", self.is_expired());
        false
    }
}

impl PenetrationLifetime {
    pub fn new(penetration: u32) -> Self {
        PenetrationLifetime {
            health: Health::new(penetration),
        }
    }
    pub fn tick(&mut self) {
        self.health.take(1);
    }
}

#[cfg(feature = "egui")]
impl Inspectable for PenetrationLifetime {
    type Attributes = ();

    fn ui(&mut self, _ui: &mut Ui, _options: Self::Attributes, _context: &mut Context) -> bool {
        println!("{}", self.is_expired());
        false
    }
}

impl Lifetime for PenetrationLifetime {
    fn is_expired(&self) -> bool {
        self.health.is_zero()
    }
    fn reset(&mut self) {
        self.health.reset()
    }
}

pub fn duration_lifetime_system(time: Res<Time>, mut query: Query<&mut DurationLifetime>) {
    for mut lifetime in query.iter_mut() {
        lifetime.timer.tick(time.delta());
    }
}

pub fn distance_lifetime_system(mut query: Query<(&Transform, &mut DistanceLifetime)>) {
    for (transform, mut lifetime) in query.iter_mut() {
        lifetime.displacement.update(transform.translation);
    }
}
