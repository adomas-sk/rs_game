use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    buildings::{
        events::{DeselectBuilding, SelectLaboratoryBuilding, SelectMinionAssemblyBuilding},
        laboratory::{
            handle_purchase_powerup, laboratory_ui, purchase_button_enablement,
            purchase_button_interaction, LaboratoryUI,
        },
        minion_assembly::{
            build_button_enablement, build_button_interaction, handle_build_gathering,
            minion_assembly_ui, MinionAssemblyUI,
        },
    },
    shared::BuildingUI,
};

use self::top_ui::{setup_top_ui, update_hydrogen_counter, update_minion_counter};

pub mod top_ui;

#[derive(Component)]
pub struct BuildingUIContainer;

pub struct UIPlugin;

// For reference, remove later

fn open_minion_assembly_ui(
    mut building_ui_query: Query<&mut Style, With<BuildingUIContainer>>,
    mut building_query: Query<&mut Style, (With<MinionAssemblyUI>, Without<BuildingUIContainer>)>,
) {
    let mut style = building_ui_query.single_mut();
    style.display = Display::Flex;

    let mut ms_style = building_query.single_mut();
    ms_style.display = Display::Flex;
}
fn open_laboratory_ui(
    mut building_ui_query: Query<&mut Style, With<BuildingUIContainer>>,
    mut building_query: Query<&mut Style, (With<LaboratoryUI>, Without<BuildingUIContainer>)>,
) {
    let mut style = building_ui_query.single_mut();
    style.display = Display::Flex;

    let mut ms_style = building_query.single_mut();
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

fn setup_ui(mut commands: Commands) {
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
            laboratory_ui(parent);
        });
}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Top
            .add_systems(Startup, setup_top_ui)
            .add_systems(Update, (update_hydrogen_counter, update_minion_counter))
            // Building UI
            .add_systems(Startup, setup_ui)
            // Minion assembly building
            .add_systems(
                Update,
                (
                    build_button_enablement,
                    build_button_interaction.after(build_button_enablement),
                ),
            )
            .add_systems(Update, handle_build_gathering)
            .add_systems(
                Update,
                open_minion_assembly_ui.run_if(on_event::<SelectMinionAssemblyBuilding>()),
            )
            // Laboratory building
            .add_systems(
                Update,
                (
                    purchase_button_enablement,
                    purchase_button_interaction.after(purchase_button_enablement),
                ),
            )
            .add_systems(Update, handle_purchase_powerup)
            .add_systems(
                Update,
                open_laboratory_ui.run_if(on_event::<SelectLaboratoryBuilding>()),
            )
            // Event handling
            .add_systems(
                Update,
                close_building_ui.run_if(on_event::<DeselectBuilding>()),
            );
    }
}
