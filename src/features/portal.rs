use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::states::{self, GameState};

use super::{player::Player, shared::despawn_component};

pub struct PortalPlugin;

#[derive(Component, Debug)]
pub struct Portal;

fn setup_portal(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Portal,
            PbrBundle {
                mesh: meshes.add(Circle::new(2.0)),
                material: materials.add(Color::rgb(0.9, 0.1, 0.1)),
                ..default()
            },
            Collider::ball(1.0),
        ))
        .insert(Sensor)
        .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC)
        .insert(TransformBundle::from(Transform::from_xyz(10.0, 1.0, 10.0)));
}

fn update_portal(
    mut transform_query: Query<&mut Transform, With<Portal>>,
    camera_query: Query<&GlobalTransform, With<Camera>>,
) {
    let camera = camera_query.single();
    for mut transform in &mut transform_query {
        transform.look_at(camera.translation(), Vec3::Y);
        transform.rotate_local_axis(Vec3::Y, PI)
    }
}

fn enter_portal(
    rapier_context: Res<RapierContext>,
    current_state: Res<State<GameState>>,
    mut game_state: ResMut<NextState<GameState>>,
    portal_query: Query<Entity, With<Portal>>,
    player_query: Query<Entity, With<Player>>,
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };
    let Ok(portal) = portal_query.get_single() else {
        return;
    };
    if rapier_context.intersection_pair(player, portal) == Some(true) {
        if *current_state.get() == states::GameState::Home {
            game_state.set(states::GameState::Arena);
        } else {
            game_state.set(states::GameState::Home);
        }
    };
}

impl Plugin for PortalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(states::GameState::Home), setup_portal)
            .add_systems(OnEnter(states::GameState::Arena), setup_portal)
            .add_systems(
                Update,
                update_portal.run_if(
                    in_state(states::GameState::Home).or_else(in_state(states::GameState::Arena)),
                ),
            )
            .add_systems(
                Update,
                enter_portal.run_if(
                    in_state(states::GameState::Home).or_else(in_state(states::GameState::Arena)),
                ),
            )
            .add_systems(OnExit(states::GameState::Home), despawn_component::<Portal>)
            .add_systems(
                OnExit(states::GameState::Arena),
                despawn_component::<Portal>,
            );
    }
}
