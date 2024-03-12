use bevy::prelude::*;

pub mod hydrogen;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hydrogen::setup_hydrogen_resource);
    }
}
