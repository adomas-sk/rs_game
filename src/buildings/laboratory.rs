use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::shared::{BuildingUI, Hydrogen, PRICES, TRANSLATIONS};

use super::{
    events::{DeselectBuilding, SelectLaboratoryBuilding},
    shared::HIGHLIGHT_TINT,
};

#[derive(Component)]
pub struct LaboratoryBuilding;

pub fn setup_laboratory_building(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(LaboratoryBuilding)
        .insert((
            PbrBundle {
                mesh: meshes.add(Cuboid {
                    half_size: Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                }),
                material: materials.add(Color::rgb(0.8, 0.5, 0.3)),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Select>>::send_event::<SelectLaboratoryBuilding>(),
            On::<Pointer<Deselect>>::send_event::<DeselectBuilding>(),
            HIGHLIGHT_TINT,
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(1.0, 1.0, 1.0))
        .insert(TransformBundle::from(Transform::from_xyz(-10.0, 0.5, 10.0)));
}

#[derive(Component)]
pub struct LaboratoryUI;

#[derive(Component)]
pub struct LaboratoryBuildButton;

#[derive(Component)]
pub struct LaboratoryBuildGathering;

pub fn laboratory_ui(parent: &mut ChildBuilder<'_>) {
    parent
        .spawn((LaboratoryUI, BuildingUI))
        .insert((
            NodeBundle {
                style: Style {
                    display: Display::None,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(12.0),
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            NoDeselect,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    TRANSLATIONS.laboratory,
                    TextStyle {
                        font_size: 20.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
                NoDeselect,
            ));
            parent
                .spawn((
                    LaboratoryBuildButton,
                    LaboratoryBuildGathering,
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(140.0),
                            height: Val::Px(60.0),
                            border: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        ..default()
                    },
                    NoDeselect,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            TRANSLATIONS.power_up,
                            TextStyle {
                                font_size: 14.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                        NoDeselect,
                    ));
                });
        });
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const DISABLED_BUTTON: Color = Color::rgb(0.4, 0.4, 0.4);
pub fn purchase_button_interaction(
    hydrogen: Res<Hydrogen>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<LaboratoryBuildButton>),
    >,
) {
    if hydrogen.0 < PRICES.power_up {
        return;
    }
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                return;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
                return;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
                return;
            }
        }
    }
}
pub fn purchase_button_enablement(
    hydrogen: Res<Hydrogen>,
    mut button_query: Query<(&mut BackgroundColor, &mut BorderColor), With<LaboratoryBuildButton>>,
) {
    for (mut color, mut border_color) in &mut button_query {
        if hydrogen.0 < PRICES.power_up {
            *color = DISABLED_BUTTON.into();
            border_color.0 = Color::BLACK;
            return;
        } else {
            *color = NORMAL_BUTTON.into();
            border_color.0 = Color::BLACK;
        }
    }
}

pub fn handle_purchase_powerup(
    mut hydrogen: ResMut<Hydrogen>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<LaboratoryBuildGathering>)>,
) {
    if hydrogen.0 < PRICES.power_up {
        return;
    }
    let Ok(interaction) = interaction_query.get_single() else {
        return;
    };
    if matches!(interaction, Interaction::Pressed) {
        hydrogen.0 -= PRICES.power_up;
        println!("You powered up son");
    }
}
