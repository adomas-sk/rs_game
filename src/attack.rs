use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct AttackPlugin;

#[derive(Component)]
pub struct Projectile {
    time_alive: Timer,
    source: Vec3,
    target: Vec3,
}

const ALIVE_TIME: f32 = 2.0;

pub fn spawn_projectile(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    source: Vec3,
    target: Vec3,
) {
    commands
        .spawn(Projectile {
            time_alive: Timer::from_seconds(ALIVE_TIME, TimerMode::Once),
            source: source,
            target: target,
        })
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.25,
                ..default()
            })),
            material: materials.add(Color::rgb_u8(229, 220, 41).into()),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_translation(source)))
        .insert(Collider::ball(0.25));
}

fn update_projectile(
    mut commands: Commands,
    mut projectiles_query: Query<(&mut Projectile, Entity, &mut Transform), With<Projectile>>,
    time: Res<Time>,
) {
    for (mut projectile, entity, mut transform) in &mut projectiles_query {
        if projectile.time_alive.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
        transform.translation = projectile.source.lerp(projectile.target, 1.0 -projectile.time_alive.percent_left());
    }
}

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_projectile);
    }
}
