use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::prelude::*;

use crate::{enemy::Enemy, player::Player, shared_resources::Gravity};

pub struct MinionPlugin;

#[derive(Component)]
pub struct Minion {
    roam_timer: Timer,
    roam_direction: Vec3,
    roam_on: bool,
}

const FOLLOW_DISTANCE: f32 = 5.0;
const ATTACK_DISTANCE: f32 = 10.0;
const SPEED: f32 = 7.0;

fn setup_minion(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Minion {
            roam_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            roam_direction: Vec3::ZERO,
            roam_on: false,
        })
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(124, 255, 124).into()),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(2.0, 2.0, 2.0)))
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController::default())
        .insert(Collider::cuboid(0.5, 0.5, 0.5));
}

fn change_roam_direction(time: Res<Time>, mut minion_query: Query<&mut Minion>) {
    for mut minion in &mut minion_query {
        if minion.roam_timer.tick(time.delta()).just_finished() {
            minion.roam_on = true;
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
        (
            &mut Minion,
            &GlobalTransform,
            &mut KinematicCharacterController,
        ),
        With<Minion>,
    >,
    enemy_query: Query<&GlobalTransform, With<Enemy>>,
    player_query: Query<&GlobalTransform, With<Player>>,
) {
    for (mut minion, minion_global_transform, mut controller) in &mut minion_query {
        let delta_gravity = gravity.0 * time.delta_seconds();

        let player_transform = match player_query.get_single() {
            Ok(player) => player,
            Err(err) => {
                println!("Error: Something wrong with player! {}", err);
                panic!();
            }
        };

        // Always follow player
        let distance = player_transform
            .translation()
            .distance(minion_global_transform.translation());
        if distance > FOLLOW_DISTANCE {
            let direction = player_transform.translation() - minion_global_transform.translation();
            let velocity = direction.normalize() * (Vec3::splat(SPEED) * time.delta_seconds());
            controller.translation = Some(velocity + delta_gravity);
            return;
        }

        // Then attack enemy
        for enemy_transform in &enemy_query {
            let distance = enemy_transform
                .translation()
                .distance(minion_global_transform.translation());
            if distance < ATTACK_DISTANCE {
                let direction =
                    enemy_transform.translation() - minion_global_transform.translation();
                let velocity = direction.normalize() * (Vec3::splat(SPEED) * time.delta_seconds());
                controller.translation = Some(velocity + delta_gravity);
                return;
            }
        }

        // If close enough, roam
        if minion.roam_on {
            let target =
                player_transform.translation() + (minion.roam_direction * (FOLLOW_DISTANCE * 0.8));
            let distance = minion_global_transform.translation().distance(target);
            if distance > 1.5 {
                let direction = target - minion_global_transform.translation();
                let velocity = direction.normalize() * (Vec3::splat(SPEED) * time.delta_seconds());
                controller.translation = Some(velocity + delta_gravity);
                return;
            } else {
                minion.roam_on = false;
            }
        }

        // Finally apply gravity
        controller.translation = Some(delta_gravity);
    }
}

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_minion)
            .add_systems(Update, (update_minion, change_roam_direction));
    }
}
