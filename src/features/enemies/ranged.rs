use bevy::{ecs::query::QuerySingleError, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::features::{player::Player, shared::Gravity};

#[derive(Component)]
pub struct RangedEnemy;

const ATTACK_DISTANCE: f32 = 10.0;
const SPEED: f32 = 7.0;

pub fn setup_ranged_enemy(
    location: Transform,
) -> impl FnMut(Commands, ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>) {
    let system = move |mut commands: Commands,
                       mut meshes: ResMut<Assets<Mesh>>,
                       mut materials: ResMut<Assets<StandardMaterial>>| {
        commands
            .spawn(RangedEnemy)
            .insert(PbrBundle {
                mesh: meshes.add(Mesh::from(Cuboid {
                    half_size: Vec3::splat(0.5),
                })),
                material: materials.add(Color::rgb_u8(255, 124, 124)),
                ..default()
            })
            .insert(TransformBundle::from(location))
            .insert(RigidBody::KinematicPositionBased)
            .insert(KinematicCharacterController::default())
            .insert(Collider::ball(0.5));
    };

    return system;
}

pub fn update_ranged_enemy(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut controllers_query: Query<&mut KinematicCharacterController, With<RangedEnemy>>,
    enemy_global_transform_query: Query<&GlobalTransform, With<RangedEnemy>>,
    player_query: Query<&GlobalTransform, With<Player>>,
    // minions_query: Query<&GlobalTransform, With<Minion>>,
) {
    let mut controller = controllers_query.single_mut();
    let enemy_global_transform = enemy_global_transform_query.single();

    let delta_gravity = gravity.0 * time.delta_seconds();

    // First attack minions
    // for minion_transform in &minions_query {
    //     let distance = minion_transform
    //         .translation()
    //         .distance(enemy_global_transform.translation());
    //     if distance < ATTACK_DISTANCE {
    //         let direction = minion_transform.translation() - enemy_global_transform.translation();
    //         let velocity = direction.normalize() * (Vec3::splat(SPEED) * time.delta_seconds());
    //         controller.translation = Some(velocity + delta_gravity);
    //         return;
    //     }
    // }

    // Then attack player
    match player_query.get_single() {
        Ok(&player_transform) => {
            let distance = player_transform
                .translation()
                .distance(enemy_global_transform.translation());
            if distance < ATTACK_DISTANCE {
                let direction =
                    player_transform.translation() - enemy_global_transform.translation();
                let velocity = direction.normalize() * (Vec3::splat(SPEED) * time.delta_seconds());
                controller.translation = Some(velocity + delta_gravity);
                return;
            }
        }
        Err(QuerySingleError::NoEntities(_)) => {
            println!("Error: There is no player!");
            panic!();
        }
        Err(QuerySingleError::MultipleEntities(_)) => {
            println!("Error: There is more than one player!");
            panic!();
        }
    }

    // Finaly just apply gravity
    controller.translation = Some(delta_gravity);
}
