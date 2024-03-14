use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    buildings::minion_assembly::{
        minion_assembly_ui, DeselectBuilding, MinionAssemblyUI, SelectBuilding,
    },
    shared::{BuildingUI, Hydrogen, TRANSLATIONS},
};

#[derive(Component)]
struct HydrogenCounter;

#[derive(Component)]
pub struct BuildingUIContainer;

pub struct UIPlugin;

// For reference, remove later
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// For reference, remove later
fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn open_building_ui(
    mut building_ui_query: Query<&mut Style, With<BuildingUIContainer>>,
    mut minion_assembly_query: Query<
        &mut Style,
        (With<MinionAssemblyUI>, Without<BuildingUIContainer>),
    >,
) {
    let mut style = building_ui_query.single_mut();
    style.display = Display::Flex;

    let mut ms_style = minion_assembly_query.single_mut();
    ms_style.display = Display::Flex;
}

fn close_building_ui(
    mut building_ui_query: Query<&mut Style, With<BuildingUIContainer>>,
    mut uis_query: Query<&mut Style, (With<BuildingUI>, Without<BuildingUIContainer>)>,
) {
    let mut style = building_ui_query.single_mut();
    style.display = Display::None;

    for mut ui_style in &mut uis_query {
        ui_style.display = Display::None;
    }
}

fn update_hydrogen_counter(
    hydrogen: Res<Hydrogen>,
    mut text_query: Query<&mut Text, With<HydrogenCounter>>,
) {
    let mut text = text_query.single_mut();
    text.sections[0].value = TRANSLATIONS.hydrogen.to_string() + &hydrogen.0.to_string();
}

fn setup_ui(mut commands: Commands) {
    // Top UI
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
                    "Resource: 0",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            // Button for reference, remove later
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(24.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
    // Building UI
    commands
        .spawn(BuildingUIContainer)
        .insert((
            NodeBundle {
                style: Style {
                    top: Val::Px(64.0),
                    display: Display::None,
                    width: Val::Px(320.0),
                    height: Val::Px(320.0),
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(12.0),
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
            },
            PickableBundle::default(),
            NoDeselect,
        ))
        .with_children(|parent| {
            minion_assembly_ui(parent);
        });
}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, button_system)
            .add_systems(Update, update_hydrogen_counter)
            .add_systems(
                Update,
                close_building_ui.run_if(on_event::<DeselectBuilding>()),
            )
            .add_systems(
                Update,
                open_building_ui.run_if(on_event::<SelectBuilding>()),
            );
    }
}
