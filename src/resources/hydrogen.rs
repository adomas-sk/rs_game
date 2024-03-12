use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct HydrogenResource;

pub fn setup_hydrogen_resource(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(HydrogenResource)
        .insert(PbrBundle {
            mesh: meshes.add(Sphere { radius: 1.0 }),
            material: materials.add(Color::BLUE),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::ball(1.0))
        .insert(TransformBundle::from(Transform::from_xyz(5.0, 0.5, 5.0)));
}
