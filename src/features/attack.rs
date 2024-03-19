use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::states;

use super::shared::despawn_component;

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
            mesh: meshes.add(Mesh::from(Sphere { radius: 0.25 })),
            material: materials.add(Color::rgb_u8(229, 220, 41)),
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
        transform.translation = projectile.source.lerp(
            projectile.target,
            1.0 - projectile.time_alive.fraction_remaining(),
        );
    }
}

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_projectile.run_if(in_state(states::GameState::Home)),
        )
        .add_systems(
            OnExit(states::GameState::Home),
            despawn_component::<Projectile>,
        );
    }
}
