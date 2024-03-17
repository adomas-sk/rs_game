use bevy::prelude::*;

use crate::shared::{Hydrogen, MinionInventory, TRANSLATIONS};

#[derive(Component)]
pub struct HydrogenCounter;

#[derive(Component)]
pub struct MinionCounter;

#[derive(Component)]
pub struct BuildingUIContainer;

pub fn setup_top_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(32.0),
                align_items: AlignItems::Center,
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
        })
        .with_children(|parent| {
            parent
                .spawn(HydrogenCounter)
                .insert(TextBundle::from_section(
                    "Hydrogen: 0",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            parent
                .spawn(MinionCounter)
                .insert(TextBundle::from_section(
                    "Minion: 0",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
        });
}

pub fn update_hydrogen_counter(
    hydrogen: Res<Hydrogen>,
    mut text_query: Query<&mut Text, With<HydrogenCounter>>,
) {
    let mut text = text_query.single_mut();
    text.sections[0].value = TRANSLATIONS.top_hydrogen.to_string() + &hydrogen.0.to_string();
}

pub fn update_minion_counter(
    minion_inventory: Res<MinionInventory>,
    mut text_query: Query<&mut Text, With<MinionCounter>>,
) {
    let mut text = text_query.single_mut();
    text.sections[0].value = TRANSLATIONS.top_minions.to_string() + &minion_inventory.gathering.to_string();
}
