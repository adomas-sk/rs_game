use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod enemy;
mod ground;
mod minion;
mod player;
mod selector;
mod shared_resources;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(shared_resources::SharedResourcesPlugin)
        .add_plugins(ground::GroundPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(selector::SelectorPlugin)
        .add_plugins(minion::MinionPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}
