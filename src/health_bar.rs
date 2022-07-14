#![warn(missing_docs)]

//! Ingame health bar UI element

use bevy::{prelude::*, sprite::Anchor};

pub struct HealthBarPrefab {
    pub dimension: Vec2,
    pub bg_color: Color,
    pub fg_color: Color,
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

pub fn spawn_health_bar(cmd: &mut Commands, prefab: HealthBarPrefab) -> Entity {
    let parent = cmd.spawn().id();
    let bg = cmd.spawn().id();
    let fg = cmd.spawn().id();

    cmd.entity(parent)
        .insert(HealthBar::new(prefab.dimension))
        .insert(Transform {
            translation: prefab.translation,
            ..default()
        });

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
