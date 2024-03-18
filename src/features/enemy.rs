use bevy::{ecs::query::QuerySingleError, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::states;

use super::{minions::following::Minion, player::Player, shared::Gravity};

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

const ATTACK_DISTANCE: f32 = 10.0;
const SPEED: f32 = 7.0;

fn setup_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Enemy)
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid {
                half_size: Vec3::splat(0.5),
            })),
            material: materials.add(Color::rgb_u8(255, 124, 124)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            -10.0, 2.0, -10.0,
        )))
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController::default())
        .insert(Collider::cuboid(0.5, 0.5, 0.5));
}

fn update_enemy(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut controllers_query: Query<&mut KinematicCharacterController, With<Enemy>>,
    enemy_global_transform_query: Query<&GlobalTransform, With<Enemy>>,
    player_query: Query<&GlobalTransform, With<Player>>,
    minions_query: Query<&GlobalTransform, With<Minion>>,
) {
    let mut controller = controllers_query.single_mut();
    let enemy_global_transform = enemy_global_transform_query.single();

    let delta_gravity = gravity.0 * time.delta_seconds();

    // First attack minions
    for minion_transform in &minions_query {
        let distance = minion_transform
            .translation()
            .distance(enemy_global_transform.translation());
        if distance < ATTACK_DISTANCE {
            let direction = minion_transform.translation() - enemy_global_transform.translation();
            let velocity = direction.normalize() * (Vec3::splat(SPEED) * time.delta_seconds());
            controller.translation = Some(velocity + delta_gravity);
            return;
        }
    }

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

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(states::GameState::Home), setup_enemy)
            .add_systems(
                Update,
                update_enemy.run_if(in_state(states::GameState::Home)),
            );
    }
}
