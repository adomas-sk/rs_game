use bevy::prelude::*;

use crate::features::shared::{PowerUpInventory, PowerUps, TRANSLATIONS};

#[derive(Component)]
pub struct PowerUpsUI;

#[derive(Component)]
pub struct PowerUpIcon(PowerUps);

pub fn setup_power_ups_ui(mut commands: Commands) {
    let create_powerup_icon = |parent: &mut ChildBuilder<'_>, powerup: PowerUps, label: &str| {
        parent
            .spawn((
                PowerUpIcon(powerup),
                NodeBundle {
                    style: Style {
                        width: Val::Px(140.0),
                        height: Val::Px(60.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::rgb(0.2, 0.2, 0.2)),
                    background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    label,
                    TextStyle {
                        font_size: 14.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            });
    };

    commands
        .spawn(PowerUpsUI)
        .insert(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(64.0),
                right: Val::Px(0.0),
                display: Display::None,
                flex_direction: FlexDirection::Column,
                width: Val::Px(320.0),
                height: Val::Px(320.0),
                align_items: AlignItems::Center,
                row_gap: Val::Px(12.0),
                padding: UiRect {
                    left: Val::Px(32.0),
                    right: Val::Px(32.0),
                    top: Val::Px(20.0),
                    bottom: Val::Px(20.0),
                },
                ..default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.6).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((TextBundle::from_section(
                TRANSLATIONS.powerup_title,
                TextStyle {
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ),));
            create_powerup_icon(parent, PowerUps::PowerUp1, TRANSLATIONS.powerup_powerup1);
            create_powerup_icon(parent, PowerUps::PowerUp2, TRANSLATIONS.powerup_powerup2);
            create_powerup_icon(parent, PowerUps::PowerUp3, TRANSLATIONS.powerup_powerup3);
        });
}

pub fn update_power_ups(
    powerup_inventory: Res<PowerUpInventory>,
    container_query: Query<&Style, With<PowerUpsUI>>,
    mut icons_query: Query<
        (&mut BackgroundColor, &mut BorderColor, &PowerUpIcon),
        With<PowerUpIcon>,
    >,
) {
    let Ok(style) = container_query.get_single() else {
        return;
    };
    if style.display == Display::None {
        return;
    }

    for (mut background_color, mut border_color, powerup) in &mut icons_query {
        let mut update_powerup_display = |powerup: bool| {
            if powerup {
                background_color.0 = Color::rgba(0.2, 0.2, 0.2, 0.0);
                border_color.0 = Color::rgb(0.9, 0.9, 0.1);
            } else {
                background_color.0 = Color::rgb(0.2, 0.2, 0.2);
                border_color.0 = Color::rgb(0.2, 0.2, 0.2);
            }
        };

        match powerup.0 {
            PowerUps::PowerUp1 => {
                update_powerup_display(powerup_inventory.powerup1);
            }
            PowerUps::PowerUp2 => {
                update_powerup_display(powerup_inventory.powerup2);
            }
            PowerUps::PowerUp3 => {
                update_powerup_display(powerup_inventory.powerup3);
            }
        }
    }
}

pub fn power_ups_toggle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut container_query: Query<&mut Style, With<PowerUpsUI>>,
) {
    let mut style = container_query.single_mut();

    if keyboard_input.just_pressed(KeyCode::KeyP) {
        if style.display == Display::None {
            style.display = Display::Flex;
        } else {
            style.display = Display::None;
        }
    }
}
