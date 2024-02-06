use bevy::prelude::*;

mod player;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, player::PlayerPlugin))
        .run();
}
