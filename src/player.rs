use std::time::Duration;

use bevy::{prelude::*, transform::TransformSystem};
use bevy_rapier3d::prelude::*;

use crate::{attack::spawn_projectile, shared_resources::Gravity};

pub struct PlayerPlugin;

#[derive(Resource)]
struct CameraOffset(Vec3);

#[derive(Resource)]
struct LightOffset(Vec3);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerModel;

#[derive(Resource)]
struct PlayerAnimations(Vec<Handle<AnimationClip>>);

const SPEED: f32 = 10.0;

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // animations
    commands.insert_resource(PlayerAnimations(vec![
        asset_server.load("player/player.glb#Animation0"),
        asset_server.load("player/player.glb#Animation1"),
    ]));

    // player
    commands
        .spawn(Player)
        .insert(InheritedVisibility::default())
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 2.0, 0.0)))
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController::default())
        .insert(Collider::ball(1.0))
        .with_children(|parent| {
            parent.spawn(PlayerModel).insert(SceneBundle {
                scene: asset_server.load("player/player.glb#Scene0"),
                transform: Transform::from_xyz(0.0, -0.5, 0.0),
                ..default()
            });
        });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 150000.0,
            range: 100.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle { ..default() });
}

fn follow_player(
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
    let player_transform = player_transforms.single();
    let mut camera_transform = camera_transforms.single_mut();
    let mut light_transform = light_transforms.single_mut();

    camera_transform.translation = player_transform.translation + camera_offset.0;
    camera_transform.look_at(player_transform.translation, Vec3::Y);

    light_transform.translation = player_transform.translation + light_offset.0;
    light_transform.look_at(player_transform.translation, Vec3::Y);
}

fn move_player(
    animations: Res<PlayerAnimations>,
    mut animation_player_query: Query<&mut AnimationPlayer>,
    mut transform_query: Query<&mut Transform, With<PlayerModel>>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    gravity: Res<Gravity>,
    mut controllers: Query<&mut KinematicCharacterController, With<Player>>,
) {
    let mut controller = controllers.single_mut();

    let mut input_direction = Vec3::ZERO;
    // Collect input
    if keyboard_input.pressed(KeyCode::KeyA) {
        input_direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        input_direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        input_direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        input_direction.z += 1.0;
    }

    let is_moving = input_direction.length() > 0.0;

    // Update animation
    for mut animation_player in &mut animation_player_query {
        if is_moving {
            if !animation_player.is_playing_clip(&animations.0[0]) {
                animation_player
                    .play_with_transition(animations.0[0].clone_weak(), Duration::from_millis(250))
                    .repeat();
            }
        } else {
            if !animation_player.is_playing_clip(&animations.0[1]) {
                animation_player
                    .play_with_transition(animations.0[1].clone_weak(), Duration::from_millis(250))
                    .repeat();
            }
        }
    }

    // rotate direction to how player sees world
    let rotated_to_camera_view_direction = Vec3 {
        x: (input_direction.x / 2.0 - input_direction.z / 2.0),
        y: 0.0,
        z: (input_direction.x / 2.0 + input_direction.z / 2.0),
    };

    // rotate model
    if is_moving {
        let mut child_transform = transform_query.single_mut();
        child_transform.look_at(
            rotated_to_camera_view_direction
                * Vec3 {
                    x: -1.0,
                    y: 0.0,
                    z: -1.0,
                }
                + Vec3 {
                    x: 0.0,
                    y: -0.5,
                    z: 0.0,
                },
            Vec3::Y,
        );
    }

    // calculate velocity and update controller
    let delta_gravity = gravity.0 * time.delta_seconds();

    if input_direction.length() < 0.01 {
        controller.translation = Some(delta_gravity);
        return;
    }

    let input_velocity =
        rotated_to_camera_view_direction.normalize() * (Vec3::splat(SPEED) * time.delta_seconds());

    let final_velocity = input_velocity + delta_gravity;

    controller.translation = Some(final_velocity);
}

fn create_projectile(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        spawn_projectile(
            commands,
            meshes,
            materials,
            Vec3 {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            },
            Vec3 {
                x: 10.0,
                y: 1.0,
                z: 10.0,
            },
        )
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vec3::NEG_Y * 5.0))
            .insert_resource(CameraOffset(Vec3 {
                x: -9.0,
                y: 14.0,
                z: 9.0,
            }))
            .insert_resource(LightOffset(Vec3 {
                x: 1.0,
                y: 4.0,
                z: 2.0,
            }))
            .add_systems(Startup, setup_player)
            .add_systems(Update, move_player)
            .add_systems(Update, create_projectile)
            .add_systems(
                PostUpdate,
                follow_player
                    .after(PhysicsSet::Writeback)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}
