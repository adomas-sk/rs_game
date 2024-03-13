use bevy::prelude::*;

use self::minion_assembly::OpenMinionAssemblyUI;

pub mod gathering_post;
pub mod minion_assembly;

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, gathering_post::setup_gathering_post_building)
            .add_systems(Startup, minion_assembly::setup_minion_assembly_building)
            .add_event::<OpenMinionAssemblyUI>();
    }
}
