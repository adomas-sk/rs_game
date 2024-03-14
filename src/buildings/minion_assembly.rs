use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::shared::BuildingUI;

#[derive(Component)]
pub struct MinionAssemblyBuilding;

const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + Color::rgba(0.2, 0.2, 0.2, 0.0),
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + Color::rgba(0.3, 0.3, 0.3, 0.0),
        ..matl.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + Color::rgba(0.15, 0.15, 0.15, 0.0),
        ..matl.to_owned()
    })),
};

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
            On::<Pointer<Select>>::send_event::<SelectBuilding>(),
            On::<Pointer<Deselect>>::send_event::<DeselectBuilding>(),
            HIGHLIGHT_TINT,
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(1.0, 1.0, 1.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.5, 10.0)));
}

#[derive(Event)]
pub struct SelectBuilding;

impl From<ListenerInput<Pointer<Select>>> for SelectBuilding {
    fn from(_: ListenerInput<Pointer<Select>>) -> Self {
        SelectBuilding
    }
}

#[derive(Event)]
pub struct DeselectBuilding;

impl From<ListenerInput<Pointer<Deselect>>> for DeselectBuilding {
    fn from(_: ListenerInput<Pointer<Deselect>>) -> Self {
        DeselectBuilding
    }
}

#[derive(Component)]
pub struct MinionAssemblyUI;

pub fn minion_assembly_ui(top_parent: &mut ChildBuilder<'_>) {
    top_parent
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
                    "Minion assembly",
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
                            "Gathering",
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
