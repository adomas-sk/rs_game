use bevy::prelude::*;

pub mod gathering_post;

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, gathering_post::setup_gathering_post_building);
    }
}
