use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::shared::{BuildingUI, Hydrogen, MinionInventory, PRICES, TRANSLATIONS};

use super::{
    events::{DeselectBuilding, SelectMinionAssemblyBuilding},
    shared::{PurchaseButton, HIGHLIGHT_TINT},
};

#[derive(Component)]
pub struct MinionAssemblyBuilding;

pub fn setup_minion_assembly_building(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(MinionAssemblyBuilding)
        .insert((
            PbrBundle {
                mesh: meshes.add(Cuboid {
                    half_size: Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                }),
                material: materials.add(Color::rgb(0.5, 0.5, 0.0)),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Select>>::send_event::<SelectMinionAssemblyBuilding>(),
            On::<Pointer<Deselect>>::send_event::<DeselectBuilding>(),
            HIGHLIGHT_TINT,
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(1.0, 1.0, 1.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.5, 10.0)));
}

#[derive(Component)]
pub struct MinionAssemblyUI;

#[derive(Component)]
pub struct MinionAssemblyBuildGathering;

pub fn minion_assembly_ui(parent: &mut ChildBuilder<'_>) {
    parent
        .spawn((MinionAssemblyUI, BuildingUI))
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
                    TRANSLATIONS.building_minion_assembly,
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
                    PurchaseButton(PRICES.gathering_minion),
                    MinionAssemblyBuildGathering,
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
                            TRANSLATIONS.building_gathering,
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
                    PurchaseButton(15),
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(60.0),
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
                            "Attacking",
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

pub fn handle_build_gathering(
    mut minion_inventory: ResMut<MinionInventory>,
    mut hydrogen: ResMut<Hydrogen>,
    interaction_query: Query<
        (&Interaction, &PurchaseButton),
        (Changed<Interaction>, With<MinionAssemblyBuildGathering>),
    >,
) {
    let Ok((interaction, price)) = interaction_query.get_single() else {
        return;
    };
    if hydrogen.0 < price.0 {
        return;
    }
    if matches!(interaction, Interaction::Pressed) {
        hydrogen.0 -= price.0;
        minion_inventory.gathering += 1;
    }
}
