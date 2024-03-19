use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]

// TODO: Split this in to Menu state and Game state
pub enum GameState {
    #[default]
    Splash,
    Home,
    Arena,
}
