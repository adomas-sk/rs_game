use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod enemy;
mod ground;
mod minions;
mod resources;
mod buildings;
mod player;
mod selector;
mod shared_resources;
mod attack;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(shared_resources::SharedResourcesPlugin)
        .add_plugins(ground::GroundPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(selector::SelectorPlugin)
        // .add_plugins(minions::following::MinionPlugin)
        .add_plugins(buildings::BuildingsPlugin)
        .add_plugins(resources::ResourcesPlugin)
        .add_plugins(minions::gathering::GatheringMinionPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(attack::AttackPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}
