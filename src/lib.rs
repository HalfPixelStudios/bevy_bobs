//! Collection of lightweight bits and bobs for use with Bevy projects.
//!
//! Some of the things that are included:
//! - system for spawning waves of enemies
//! - dead simple 2d physics system
//! - prefab loader from RON files
//! - a handful of useful components like health and lifetime for bullets
//!
//! All of these utilities can be included by enabling their feature flag.
//!
//! In addition, the `serde` feature flag can be used to add serialization and deserialization
//! support and the `egui` flag can be enabled to allow components to be inspected using
//! [bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui)

#[cfg(feature = "attack_pattern")]
pub mod attack_pattern;
pub mod component;
#[cfg(feature = "cursor")]
pub mod cursor;
#[cfg(feature = "health_bar")]
pub mod health_bar;
pub mod misc;
#[cfg(feature = "physics_2d")]
pub mod physics_2d;
#[cfg(feature = "prefab")]
pub mod prefab;
#[cfg(feature = "sfx")]
pub mod sfx;
#[cfg(feature = "spawn_wave")]
pub mod spawn_wave;
