use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;

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

impl From<ListenerInput<Pointer<Click>>> for OpenMinionAssemblyUI {
    fn from(_: ListenerInput<Pointer<Click>>) -> Self {
        OpenMinionAssemblyUI
    }
}

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
            On::<Pointer<Click>>::send_event::<OpenMinionAssemblyUI>(),
            HIGHLIGHT_TINT,
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(1.0, 1.0, 1.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.5, 10.0)));
}

#[derive(Event)]
pub struct OpenMinionAssemblyUI;
