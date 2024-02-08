use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct GroundPlugin;

#[derive(Component)]
pub struct Ground;

fn setup_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Ground)
        .insert(PbrBundle {
            mesh: meshes.add(shape::Box::new(40.0, 1.0, 40.0).into()),
            material: materials.add(Color::WHITE.into()),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(20.0, 0.5, 20.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));

    // cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(25, 25, 255).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5));
}

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ground);
    }
}
