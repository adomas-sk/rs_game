use bevy::prelude::*;

use self::events::{DeselectBuilding, SelectLaboratoryBuilding, SelectMinionAssemblyBuilding};

pub mod events;
pub mod gathering_post;
pub mod laboratory;
pub mod minion_assembly;
pub mod shared;

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, gathering_post::setup_gathering_post_building)
            .add_systems(Startup, minion_assembly::setup_minion_assembly_building)
            .add_systems(Startup, laboratory::setup_laboratory_building)
            .add_event::<DeselectBuilding>()
            .add_event::<SelectMinionAssemblyBuilding>()
            .add_event::<SelectLaboratoryBuilding>();
    }
}
