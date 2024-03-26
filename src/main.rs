use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_rapier3d::prelude::*;

mod features;
mod states;

fn main() {
    App::new()
        .init_state::<states::GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        // Splash
        .add_plugins(features::splash::SplashPlugin)
        // Game
        .add_plugins(features::portal::PortalPlugin)
        .add_plugins(features::shared::SharedResourcesPlugin)
        .add_plugins(features::ground::GroundPlugin)
        .add_plugins(features::player::PlayerPlugin)
        .add_plugins(features::selector::SelectorPlugin)
        .add_plugins(features::buildings::BuildingsPlugin)
        .add_plugins(features::resources::ResourcesPlugin)
        .add_plugins(features::minions::gathering::GatheringMinionPlugin)
        .add_plugins(features::enemies::EnemyPlugin)
        .add_plugins(features::attack::AttackPlugin)
        .add_plugins(features::ui::UIPlugin)
        .run();
}
