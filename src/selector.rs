use bevy::prelude::*;

use crate::ground::Ground;

pub struct SelectorPlugin;

#[derive(Component)]
struct Selector;

fn draw_selector(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    ground_query: Query<&GlobalTransform, With<Ground>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();
    let ground = ground_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let Some(distance) = ray.intersect_plane(ground.translation(), Plane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);

    let snapped_point = point.round();

    // Draw a rect just above the ground plane at that position.
    gizmos.rect(
        snapped_point + ground.up() * 0.51 + ground.back() * 0.5,
        Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        Vec2::splat(1.0),
        Color::WHITE,
    );
}

impl Plugin for SelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_selector);
    }
}
