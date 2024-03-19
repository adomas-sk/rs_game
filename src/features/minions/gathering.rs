use crate::features::shared::{despawn_component, Hydrogen};
use crate::states;

use super::super::buildings::gathering_post::GatheringPostBuilding;
use super::super::resources::hydrogen::HydrogenResource;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct GatheringMinionPlugin;

#[derive(Component)]
struct GatheringMinionModel;

#[derive(Debug)]
enum GatheringMinionState {
    GoingToResource = 0,
    Gathering = 1,
    GoingToPost = 2,
    Depositing = 3,
}

#[derive(Component)]
pub struct GatheringMinion {
    work_timer: Timer,
    state: GatheringMinionState,
    resource_gathered: u32,
}

const MAX_GATHERED: u32 = 15;
const SPEED: f32 = 7.0;

fn setup_minion(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(GatheringMinion {
            work_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            state: GatheringMinionState::GoingToResource,
            resource_gathered: 0,
        })
        .insert(InheritedVisibility::default())
        .insert(TransformBundle::from(Transform::from_xyz(2.0, 1.0, 2.0)))
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController::default())
        .insert(Collider::ball(0.5))
        .insert(Sensor)
        .with_children(|parent| {
            parent.spawn(GatheringMinionModel).insert(SceneBundle {
                scene: asset_server.load("minion.glb#Scene0"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3 {
                    x: 0.5,
                    y: 0.5,
                    z: 0.5,
                }),
                ..default()
            });
        });
}

fn find_and_go_to<'a, I>(
    target_transform_query: I,
    minion: &mut GatheringMinion,
    minion_glob_transform: &GlobalTransform,
    controller: &mut KinematicCharacterController,
    time: Time,
    next_state: GatheringMinionState,
) where
    I: Iterator<Item = &'a GlobalTransform>,
{
    let (nearest_resource, distance) = {
        let mut closest_resource_transform: Option<&GlobalTransform> = None;
        let mut smallest_distance: f32 = f32::MAX;
        for resource in target_transform_query {
            let distance = minion_glob_transform
                .translation()
                .distance(resource.translation());
            if distance < smallest_distance {
                closest_resource_transform = Some(resource);
                smallest_distance = distance;
            }
        }
        (closest_resource_transform, smallest_distance)
    };

    // Go to nearest resource
    match nearest_resource {
        Some(transform) => {
            // If closeby, start gathering
            if distance < 2.0 {
                minion.state = next_state;
            }
            let direction =
                (transform.translation() - minion_glob_transform.translation()).normalize();
            controller.translation = Some(direction * SPEED * time.delta_seconds());
        }
        None => {
            println!("Could not find nearest resource");
            return;
        }
    }
}

fn update_minion(
    time: Res<Time>,
    mut hydrogen: ResMut<Hydrogen>,
    mut minion_query: Query<(
        &mut GatheringMinion,
        &mut KinematicCharacterController,
        &GlobalTransform,
    )>,
    resource_transform_query: Query<&GlobalTransform, With<HydrogenResource>>,
    gathering_post_transform_query: Query<&GlobalTransform, With<GatheringPostBuilding>>,
) {
    for (mut minion, mut controller, minion_glob_transform) in minion_query.iter_mut() {
        match minion.state {
            GatheringMinionState::GoingToResource => {
                find_and_go_to(
                    resource_transform_query.iter(),
                    &mut minion,
                    minion_glob_transform,
                    &mut controller,
                    *time,
                    GatheringMinionState::Gathering,
                );
            }
            GatheringMinionState::Gathering => {
                if minion.work_timer.tick(time.delta()).just_finished() {
                    minion.resource_gathered += 1;
                    if minion.resource_gathered == MAX_GATHERED {
                        minion.state = GatheringMinionState::GoingToPost;
                    }
                }
            }
            GatheringMinionState::GoingToPost => {
                find_and_go_to(
                    gathering_post_transform_query.iter(),
                    &mut minion,
                    minion_glob_transform,
                    &mut controller,
                    *time,
                    GatheringMinionState::Depositing,
                );
            }
            GatheringMinionState::Depositing => {
                if minion.work_timer.tick(time.delta()).just_finished() {
                    minion.resource_gathered -= 1;
                    hydrogen.0 += 1;
                    if minion.resource_gathered == 0 {
                        minion.state = GatheringMinionState::GoingToResource;
                    }
                }
            }
        }
    }
}

impl Plugin for GatheringMinionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(states::GameState::Home), setup_minion)
            .add_systems(
                Update,
                update_minion.run_if(in_state(states::GameState::Home)),
            )
            .add_systems(
                OnExit(states::GameState::Home),
                despawn_component::<GatheringMinion>,
            );
    }
}
