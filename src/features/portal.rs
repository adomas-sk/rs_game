use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::states;

use super::shared::despawn_component;

pub struct PortalPlugin;

#[derive(Component)]
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
            RigidBody::Fixed,
            Collider::ball(1.0),
        ))
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

impl Plugin for PortalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(states::GameState::Home), setup_portal)
            .add_systems(
                Update,
                update_portal.run_if(in_state(states::GameState::Home)),
            )
            .add_systems(OnExit(states::GameState::Home), despawn_component::<Portal>);
    }
}
