use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::states::{self, GameState};

use super::shared::despawn_component;

pub struct GroundPlugin;

#[derive(Component)]
pub struct Ground;

fn setup_ground(
    current_state: Res<State<GameState>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = match *current_state.get() {
        GameState::Splash => Vec3 {
            x: 20.0,
            y: 0.5,
            z: 20.0,
        },
        GameState::Home => Vec3 {
            x: 20.0,
            y: 0.5,
            z: 20.0,
        },
        GameState::Arena => Vec3 {
            x: 50.0,
            y: 0.5,
            z: 50.0,
        },
    };
    commands
        .spawn(Ground)
        .insert(PbrBundle {
            mesh: meshes.add(Cuboid { half_size: size }),
            material: materials.add(Color::WHITE),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(size.x, size.y, size.z))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(states::GameState::Home), setup_ground)
            .add_systems(OnExit(states::GameState::Home), despawn_component::<Ground>)
            .add_systems(OnEnter(states::GameState::Arena), setup_ground)
            .add_systems(
                OnExit(states::GameState::Arena),
                despawn_component::<Ground>,
            );
    }
}
