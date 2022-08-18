//! Collection of camera related utilies

use bevy::prelude::*;

/// Tag component to identify the main camera
///
/// The cursor system will grab the cursor position from the main camera
#[derive(Component)]
struct MainCamera;

/// Resource that can be used to obtain the world space cursor position
#[derive(Deref)]
pub struct Cursor(pub Vec2);

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Cursor(Vec2::ZERO))
            .add_system(cursor_system);
    }
}

fn cursor_system(
    windows: Res<Windows>,
    query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut cursor: ResMut<Cursor>,
) {
    let (camera, transform) = query.single();

    let win = windows.get_primary().unwrap();

    if let Some(pos) = win.cursor_position() {
        let window_size = Vec2::new(win.width() as f32, win.height() as f32);
        let ndc = (pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        cursor.0 = world_pos.truncate();
    }
}
