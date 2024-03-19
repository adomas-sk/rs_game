use bevy::prelude::*;

use crate::states;

use self::{
    events::{DeselectBuilding, SelectLaboratoryBuilding, SelectMinionAssemblyBuilding},
    gathering_post::GatheringPostBuilding,
    laboratory::LaboratoryBuilding,
    minion_assembly::MinionAssemblyBuilding,
};

use super::shared::despawn_component;

pub mod events;
pub mod gathering_post;
pub mod laboratory;
pub mod minion_assembly;
pub mod shared;

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(states::GameState::Home),
            gathering_post::setup_gathering_post_building,
        )
        .add_systems(
            OnEnter(states::GameState::Home),
            minion_assembly::setup_minion_assembly_building,
        )
        .add_systems(
            OnEnter(states::GameState::Home),
            laboratory::setup_laboratory_building,
        )
        .add_event::<DeselectBuilding>()
        .add_event::<SelectMinionAssemblyBuilding>()
        .add_event::<SelectLaboratoryBuilding>()
        .add_systems(
            OnExit(states::GameState::Home),
            despawn_component::<GatheringPostBuilding>,
        )
        .add_systems(
            OnExit(states::GameState::Home),
            despawn_component::<MinionAssemblyBuilding>,
        )
        .add_systems(
            OnExit(states::GameState::Home),
            despawn_component::<LaboratoryBuilding>,
        );
    }
}
