use bevy::prelude::*;

use crate::states::{self, GameState};

use super::shared::despawn_component;

#[derive(Component)]
pub struct Splash;

#[derive(Component)]
pub struct SplashStartGameButton;

#[derive(Component)]
pub struct SplashCamera;

pub struct SplashPlugin;

fn setup_splash(mut commands: Commands) {
    commands
        .spawn((
            Splash,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(12.0),
                    padding: UiRect {
                        left: Val::Px(64.0),
                        right: Val::Px(64.0),
                        top: Val::Px(24.0),
                        bottom: Val::Px(24.0),
                    },
                    ..default()
                },
                background_color: Color::rgba(0.1, 0.1, 0.1, 0.5).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    SplashStartGameButton,
                    ButtonBundle {
                        style: Style {
                            width: Val::Auto,
                            height: Val::Px(60.0),
                            border: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect {
                                left: Val::Px(20.0),
                                right: Val::Px(20.0),
                                top: Val::Px(12.0),
                                bottom: Val::Px(12.0),
                            },
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgba(0.2, 0.2, 0.2, 0.9)),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start Game",
                        TextStyle {
                            font_size: 24.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
    commands.spawn((SplashCamera, Camera2dBundle::default()));
}

fn listen_for_start_game(
    mut game_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<SplashStartGameButton>),
    >,
) {
    for (interaction, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => game_state.set(states::GameState::Home),
            Interaction::Hovered => *border_color = BorderColor(Color::rgb(0.9, 0.9, 0.3)),
            Interaction::None => *border_color = BorderColor(Color::rgba(0.0, 0.0, 0.0, 0.0)),
        }
    }
}

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_splash)
            .add_systems(
                Update,
                listen_for_start_game.run_if(in_state(states::GameState::Splash)),
            )
            .add_systems(
                OnExit(states::GameState::Splash),
                despawn_component::<Splash>,
            )
            .add_systems(
                OnExit(states::GameState::Splash),
                despawn_component::<SplashCamera>,
            );
    }
}
