use bevy::{prelude::*, transform::TransformSystem};
use bevy_rapier3d::prelude::*;

use crate::shared_resources::Gravity;

pub struct PlayerPlugin;

#[derive(Resource)]
struct CameraOffset(Vec3);

#[derive(Resource)]
struct LightOffset(Vec3);

#[derive(Component)]
pub struct Player;

const SPEED: f32 = 10.0;

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // player
    commands
        .spawn(Player)
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 2.0, 0.0)))
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController::default())
        .insert(Collider::cuboid(0.5, 0.5, 0.5));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
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
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    gravity: Res<Gravity>,
    mut controllers: Query<&mut KinematicCharacterController, With<Player>>,
) {
    let mut controller = controllers.single_mut();

    let mut input_direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::A) {
        input_direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        input_direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
        input_direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        input_direction.z += 1.0;
    }

    let delta_gravity = gravity.0 * time.delta_seconds();

    if input_direction.length() < 0.01 {
        controller.translation = Some(delta_gravity);
        return;
    }

    // TODO: Rest of this should probably be in FixedUpdate
    let input_velocity = input_direction.normalize() * (Vec3::splat(SPEED) * time.delta_seconds());

    let final_velocity = Vec3 {
        x: (input_velocity.x / 2.0 - input_velocity.z / 2.0) + delta_gravity.x,
        y: 0.0 + delta_gravity.y,
        z: (input_velocity.x / 2.0 + input_velocity.z / 2.0) + delta_gravity.z,
    };

    controller.translation = Some(final_velocity);
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
                y: 10.0,
                z: 4.0,
            }))
            .add_systems(Startup, setup_player)
            .add_systems(Update, move_player)
            .add_systems(
                PostUpdate,
                follow_player
                    .after(PhysicsSet::Writeback)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}
