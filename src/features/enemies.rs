use bevy::prelude::*;

use crate::states;

use super::shared::despawn_component;

pub mod ranged;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(states::GameState::Arena),
            ranged::spawn_ranged_enemies,
        )
        .add_systems(
            Update,
            ranged::update_ranged_enemy.run_if(in_state(states::GameState::Arena)),
        )
        .add_systems(
            OnExit(states::GameState::Arena),
            despawn_component::<ranged::RangedEnemy>,
        );
    }
}
