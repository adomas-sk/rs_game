use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::ground::Ground;

pub struct PlayerPlugin;

#[derive(Resource)]
struct CameraOffset(Vec3);

#[derive(Resource)]
struct LightOffset(Vec3);

#[derive(Component)]
struct Player {
    input_direction: Vec3,
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // player
    commands.spawn((
        Player {
            input_direction: Vec3::ZERO,
        },
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
    ));
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(25, 25, 255).into()),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    });
    // rigid body
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 2.0, 0.0)));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-9.0, 9.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn follow_player_with_camera(
    player_transforms: Query<&Transform, With<Player>>,
    camera_offset: Res<CameraOffset>,
    mut camera_transforms: Query<
        &mut Transform,
        (With<Camera3d>, Without<Player>, Without<PointLight>),
    >,
    light_offset: Res<LightOffset>,
    mut light_transforms: Query<
        &mut Transform,
        (With<PointLight>, Without<Player>, Without<Camera3d>),
    >,
) {
    let player_transform = player_transforms.get_single().unwrap();
    let mut camera_transform = camera_transforms.get_single_mut().unwrap();
    let mut light_transform = light_transforms.get_single_mut().unwrap();

    camera_transform.translation = player_transform.translation + camera_offset.0;
    camera_transform.look_at(player_transform.translation, Vec3::Y);

    light_transform.translation = player_transform.translation + light_offset.0;
}

fn walk_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_transforms: Query<&mut Transform, With<Player>>,
    mut players: Query<&mut Player>,
) {
    let mut player = players.single_mut();
    player.input_direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
        player.input_direction.x -= 1.
    }
    if keyboard_input.pressed(KeyCode::D) {
        player.input_direction.x += 1.
    }
    if keyboard_input.pressed(KeyCode::W) {
        player.input_direction.z -= 1.
    }
    if keyboard_input.pressed(KeyCode::S) {
        player.input_direction.z += 1.;
    }

    if player.input_direction.length() < 0.01 {
        return;
    }

    let normalized_direction =
        player.input_direction.normalize() * (Vec3::splat(10.) * time.delta_seconds());

    let mut transform = player_transforms.single_mut();
    transform.translation.x += normalized_direction.x / 2.0 - normalized_direction.z / 2.0;
    transform.translation.z += normalized_direction.x / 2.0 + normalized_direction.z / 2.0;
}

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

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) = ray.intersect_plane(ground.translation(), ground.back()) else {
        return;
    };
    let point = ray.get_point(distance);

    let snapped_point = point.round();

    // Draw a circle just above the ground plane at that position.
    gizmos.rect(
        snapped_point + ground.back() * 0.01,
        Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        Vec2::splat(1.0),
        Color::WHITE,
    );
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraOffset(Vec3 {
            x: -9.0,
            y: 9.0,
            z: 9.0,
        }))
        .insert_resource(LightOffset(Vec3 {
            x: 1.0,
            y: 10.0,
            z: 4.0,
        }))
        .add_systems(Startup, setup_player)
        .add_systems(
            Update,
            (
                (walk_player, follow_player_with_camera).chain(),
                draw_selector,
            ),
        );
    }
}
