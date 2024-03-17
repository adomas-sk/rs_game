use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::shared::{BuildingUI, Hydrogen, PowerUpInventory, PowerUps, PRICES, TRANSLATIONS};

use super::{
    events::{DeselectBuilding, SelectLaboratoryBuilding},
    shared::{PurchaseButton, HIGHLIGHT_TINT},
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
pub struct LaboratoryPurchase(PowerUps);

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
                    TRANSLATIONS.building_laboratory,
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
                    PurchaseButton(PRICES.power_up),
                    LaboratoryPurchase(PowerUps::PowerUp1),
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
                            TRANSLATIONS.building_powerup,
                            TextStyle {
                                font_size: 14.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                        NoDeselect,
                    ));
                });
            parent
                .spawn((
                    PurchaseButton(PRICES.power_up),
                    LaboratoryPurchase(PowerUps::PowerUp2),
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
                            TRANSLATIONS.building_powerup,
                            TextStyle {
                                font_size: 14.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ),
                        NoDeselect,
                    ));
                });
            parent
                .spawn((
                    PurchaseButton(PRICES.power_up),
                    LaboratoryPurchase(PowerUps::PowerUp3),
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
                            TRANSLATIONS.building_powerup,
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

pub fn handle_purchase_powerup(
    mut powerup_inventory: ResMut<PowerUpInventory>,
    mut hydrogen: ResMut<Hydrogen>,
    interaction_query: Query<(&Interaction, &LaboratoryPurchase, &PurchaseButton), (Changed<Interaction>, With<LaboratoryPurchase>)>,
) {
    let Ok((interaction, powerup, price)) = interaction_query.get_single() else {
        return;
    };
    if hydrogen.0 < price.0 {
        return;
    }
    if matches!(interaction, Interaction::Pressed) {
        hydrogen.0 -= price.0;
        match powerup.0 {
            PowerUps::PowerUp1 => {
                powerup_inventory.powerup1 = true;
            }
            PowerUps::PowerUp2 => {
                powerup_inventory.powerup2 = true;
            }
            PowerUps::PowerUp3 => {
                powerup_inventory.powerup3 = true;
            }
        }
    }
}
