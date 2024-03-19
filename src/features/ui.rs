use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    features::{
        buildings::{
            events::{DeselectBuilding, SelectLaboratoryBuilding, SelectMinionAssemblyBuilding},
            laboratory::{handle_purchase_powerup, laboratory_ui, LaboratoryUI},
            minion_assembly::{handle_build_gathering, minion_assembly_ui, MinionAssemblyUI},
            shared::{purchase_button_enablement, purchase_button_interaction},
        },
        shared::BuildingUI,
    },
    states,
};

use self::{
    power_ups::{power_ups_toggle, setup_power_ups_ui, update_power_ups, PowerUpsUI},
    top_ui::{setup_top_ui, update_hydrogen_counter, update_minion_counter, TopUI},
};

use super::shared::despawn_component;

pub mod power_ups;
pub mod top_ui;

#[derive(Component)]
pub struct BuildingUIContainer;

pub struct UIPlugin;

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

fn setup_buildings_ui(mut commands: Commands) {
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
            .add_systems(OnEnter(states::GameState::Home), setup_top_ui)
            .add_systems(
                Update,
                (
                    update_hydrogen_counter.run_if(in_state(states::GameState::Home)),
                    update_minion_counter.run_if(in_state(states::GameState::Home)),
                ),
            )
            .add_systems(OnExit(states::GameState::Home), despawn_component::<TopUI>)
            // Building UI
            .add_systems(OnEnter(states::GameState::Home), setup_buildings_ui)
            // Minion assembly building
            .add_systems(
                Update,
                (
                    purchase_button_enablement.run_if(in_state(states::GameState::Home)),
                    purchase_button_interaction
                        .after(purchase_button_enablement)
                        .run_if(in_state(states::GameState::Home)),
                ),
            )
            .add_systems(
                Update,
                handle_build_gathering.run_if(in_state(states::GameState::Home)),
            )
            .add_systems(
                Update,
                open_minion_assembly_ui
                    .run_if(on_event::<SelectMinionAssemblyBuilding>())
                    .run_if(in_state(states::GameState::Home)),
            )
            // Laboratory building
            .add_systems(
                Update,
                handle_purchase_powerup.run_if(in_state(states::GameState::Home)),
            )
            .add_systems(
                Update,
                open_laboratory_ui
                    .run_if(on_event::<SelectLaboratoryBuilding>())
                    .run_if(in_state(states::GameState::Home)),
            )
            .add_systems(
                OnExit(states::GameState::Home),
                despawn_component::<BuildingUIContainer>,
            )
            // Power ups
            .add_systems(OnEnter(states::GameState::Home), setup_power_ups_ui)
            .add_systems(
                Update,
                (
                    power_ups_toggle.run_if(in_state(states::GameState::Home)),
                    update_power_ups.run_if(in_state(states::GameState::Home)),
                ),
            )
            .add_systems(
                OnExit(states::GameState::Home),
                despawn_component::<PowerUpsUI>,
            )
            // Shared
            // Event handling
            .add_systems(
                Update,
                close_building_ui
                    .run_if(on_event::<DeselectBuilding>())
                    .run_if(in_state(states::GameState::Home)),
            );
    }
}
