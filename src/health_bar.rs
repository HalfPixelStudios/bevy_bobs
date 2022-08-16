//! Ingame health bar UI element

use bevy::{prelude::*, sprite::Anchor};

use crate::component::health::Health;

/// Used to initalize a health bar
pub struct HealthBarPrefab {
    /// Physical dimensions of the health bar
    pub dimension: Vec2,
    /// Color of the health bar background
    pub bg_color: Color,
    /// Color of the health bar foreground
    pub fg_color: Color,
    /// Initial location of the health bar
    pub translation: Vec3,
}

#[derive(Component)]
pub struct HealthBar {
    percent: f32,
    dimension: Vec2,
}

impl HealthBar {
    pub fn set_percent(&mut self, percent: f32) {
        self.percent = percent.clamp(0.0, 1.0);
    }
    pub fn add_percent(&mut self, percent: f32) {
        self.percent = (self.percent + percent).clamp(0.0, 1.0);
    }
}

impl HealthBar {
    pub fn new(dimension: Vec2) -> Self {
        HealthBar {
            percent: 1.,
            dimension,
        }
    }
}

#[derive(Component)]
struct HealthBarForeground;

#[derive(Component)]
struct HealthBarBackground;

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(health_bar_system);
    }
}

/// Spawn a new health bar from prefab
pub fn spawn_health_bar(cmd: &mut Commands, prefab: HealthBarPrefab) -> Entity {
    let parent = cmd.spawn().id();
    let bg = cmd.spawn().id();
    let fg = cmd.spawn().id();

    cmd.entity(parent)
        .insert(HealthBar::new(prefab.dimension))
        .insert_bundle(SpatialBundle::from_transform(Transform::from_translation(
            prefab.translation,
        )));

    cmd.entity(parent).push_children(&[fg, bg]);

    cmd.entity(bg)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: prefab.bg_color,
                custom_size: Some(prefab.dimension),
                anchor: Anchor::CenterLeft,
                ..default()
            },
            ..default()
        })
        .insert(HealthBarBackground);

    cmd.entity(fg)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: prefab.fg_color,
                custom_size: Some(prefab.dimension),
                anchor: Anchor::CenterLeft,
                ..default()
            },
            ..default()
        })
        .insert(HealthBarForeground);

    return parent;
}

fn health_bar_system(
    parent_query: Query<(&HealthBar, &Children)>,
    mut fg_query: Query<&mut Sprite, (With<HealthBarForeground>, Without<HealthBarBackground>)>,
    mut bg_query: Query<&mut Sprite, (With<HealthBarBackground>, Without<HealthBarForeground>)>,
) {
    for (health_bar, children) in parent_query.iter() {
        for &child in children.iter() {
            if let Ok(mut sprite) = fg_query.get_mut(child) {
                sprite.custom_size = Some(Vec2::new(
                    health_bar.dimension.x * health_bar.percent,
                    health_bar.dimension.y,
                ));
            }
            if let Ok(mut sprite) = bg_query.get_mut(child) {
                sprite.custom_size =
                    Some(Vec2::new(health_bar.dimension.x, health_bar.dimension.y));
            }
        }
    }
}

/*
/// Utility system that syncs bevy_bobs's health bar with health component.
pub fn sync_health(mut query: Query<(&Health, &mut HealthBar)>) {
    for (health, mut health_bar) in query.iter_mut() {
        health_bar.set_percent(health.percent());
    }
}
*/
