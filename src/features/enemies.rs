use bevy::prelude::*;

use crate::states;

use super::shared::despawn_component;

pub mod ranged;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(states::GameState::Home),
            ranged::setup_ranged_enemy(Transform::from_xyz(-10.0, 2.0, -10.0)),
        )
        .add_systems(
            Update,
            ranged::update_ranged_enemy.run_if(in_state(states::GameState::Home)),
        )
        .add_systems(
            OnExit(states::GameState::Home),
            despawn_component::<ranged::RangedEnemy>,
        );
    }
}
