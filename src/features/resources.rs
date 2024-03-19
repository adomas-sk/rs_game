use bevy::prelude::*;

use crate::states;

use super::shared::despawn_component;

pub mod hydrogen;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(states::GameState::Home),
            hydrogen::setup_hydrogen_resource,
        )
        .add_systems(
            OnExit(states::GameState::Home),
            despawn_component::<hydrogen::HydrogenResource>,
        );
    }
}
