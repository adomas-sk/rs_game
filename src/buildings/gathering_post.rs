use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct GatheringPostBuilding;

pub fn setup_gathering_post_building(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(GatheringPostBuilding)
        .insert(PbrBundle {
            mesh: meshes.add(Cuboid {
                half_size: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            }),
            material: materials.add(Color::BLUE),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(1.0, 1.0, 1.0))
        .insert(TransformBundle::from(Transform::from_xyz(-5.0, 0.5, 7.0)));
}
