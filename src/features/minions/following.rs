use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::prelude::*;

use crate::{features::enemies::Enemy, features::player::Player, features::shared::Gravity};

pub struct MinionPlugin;

#[derive(Component)]
struct MinionModel;

#[derive(Component)]
pub struct Minion {
    roam_timer: Timer,
    roam_direction: Vec3,
}

const FOLLOW_DISTANCE: f32 = 5.0;
const ATTACK_DISTANCE: f32 = 10.0;
const SPEED: f32 = 7.0;

fn setup_minion(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Minion {
            roam_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            roam_direction: Vec3::ZERO,
        })
        .insert(InheritedVisibility::default())
        .insert(TransformBundle::from(Transform::from_xyz(2.0, 1.0, 2.0)))
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController::default())
        .insert(Collider::ball(0.5))
        .with_children(|parent| {
            parent.spawn(MinionModel).insert(SceneBundle {
                scene: asset_server.load("minion.glb#Scene0"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3 {
                    x: 0.5,
                    y: 0.5,
                    z: 0.5,
                }),
                ..default()
            });
        });
}

fn change_roam_direction(time: Res<Time>, mut minion_query: Query<&mut Minion>) {
    for mut minion in &mut minion_query {
        if minion.roam_timer.tick(time.delta()).just_finished() {
            minion.roam_direction = Vec3 {
                x: (random::<f32>() * 2.0) - 1.0,
                y: 0.0,
                z: (random::<f32>() * 2.0) - 1.0,
            }
            .normalize();
        }
    }
}

fn update_minion(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut minion_query: Query<
        (&Minion, &GlobalTransform, &mut KinematicCharacterController),
        With<Minion>,
    >,
    mut model_transform_query: Query<&mut Transform, With<MinionModel>>,
    enemy_query: Query<&GlobalTransform, With<Enemy>>,
    player_query: Query<&GlobalTransform, With<Player>>,
) {
    for (minion, minion_global_transform, mut controller) in &mut minion_query {
        let delta_gravity = gravity.0 * time.delta_seconds();

        let player_transform = player_query.single();
        let mut model_transform = model_transform_query.single_mut();

        // Always follow player
        let distance = player_transform
            .translation()
            .distance(minion_global_transform.translation());
        if distance > FOLLOW_DISTANCE {
            let direction = (player_transform.translation()
                - minion_global_transform.translation())
            .normalize();
            let velocity = direction * (Vec3::splat(SPEED) * time.delta_seconds());
            controller.translation = Some(velocity + delta_gravity);
            rotate_model(&mut model_transform, &direction);
            return;
        }

        // Then attack enemy
        for enemy_transform in &enemy_query {
            let distance = enemy_transform
                .translation()
                .distance(minion_global_transform.translation());
            if distance < ATTACK_DISTANCE {
                let direction = (enemy_transform.translation()
                    - minion_global_transform.translation())
                .normalize();
                let velocity = direction * (Vec3::splat(SPEED) * time.delta_seconds());
                controller.translation = Some(velocity + delta_gravity);
                rotate_model(&mut model_transform, &direction);
                return;
            }
        }

        // If close enough, roam
        let target =
            player_transform.translation() + (minion.roam_direction * (FOLLOW_DISTANCE * 0.8));
        let distance = minion_global_transform.translation().distance(target);
        if distance > 1.5 {
            let direction = (target - minion_global_transform.translation()).normalize();
            let velocity = direction * (Vec3::splat(SPEED) * time.delta_seconds());
            controller.translation = Some(velocity + delta_gravity);
            rotate_model(&mut model_transform, &direction);
            return;
        }

        // Finally apply gravity
        controller.translation = Some(delta_gravity);
    }
}

fn rotate_model(model_transform: &mut Transform, direction: &Vec3) {
    if direction.length() <= 0.0 {
        return;
    }
    model_transform.look_at(
        Vec3 {
            x: direction.z,
            y: 0.0,
            z: -direction.x,
        },
        Vec3::Y,
    );
}

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_minion)
            .add_systems(Update, (update_minion, change_roam_direction));
    }
}
