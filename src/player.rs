use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
struct Player {
    input_direction: Vec3,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player {
            input_direction: Vec3::ZERO,
        },
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
    ));
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn follow_player_with_camera(
    player_transforms: Query<&Transform, With<Player>>,
    mut camera_transforms: Query<&mut Transform, With<Camera>>,
) {
    let player_transform = player_transforms.get_single().unwrap();
    let mut camera_transform = camera_transforms.get_single_mut().unwrap();

    let offset = Vec3 {
        x: -2.5,
        y: 4.5,
        z: 9.0,
    };
    // let camera_position = offset + player_transform.
    // camera_transform.
    // .looking_at(Vec3::ZERO, Vec3::Y)
}

fn walk_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_transforms: Query<&mut Transform, With<Player>>,
    mut players: Query<&mut Player>,
) {
    let mut player = players.get_single_mut().unwrap();
    player.input_direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
        player.input_direction.x -= 1.
    }
    if keyboard_input.pressed(KeyCode::D) {
        player.input_direction.x += 1.
    }
    if keyboard_input.pressed(KeyCode::W) {
        player.input_direction.z -= 1.
    }
    if keyboard_input.pressed(KeyCode::S) {
        player.input_direction.z += 1.;
    }

    if player.input_direction.length() < 0.01 {
        return;
    }

    let normalized_direction =
        player.input_direction.normalize() * (Vec3::splat(10.) * time.delta_seconds());

    match player_transforms.get_single_mut() {
        Err(error) => println!("Can't find player transform: {}", error),
        Ok(mut transform) => {
            transform.translation.x += normalized_direction.x;
            transform.translation.z += normalized_direction.z;
        }
    }
}

// fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
//     if timer.0.tick(time.delta()).just_finished() {
//         for name in &query {
//             println!("hello {}!", name.0);
//         }
//     }
// }

// fn update_people(mut query: Query<&mut Name, With<Person>>) {
//     for mut name in &mut query {
//         if name.0 == "Elaina Proctor" {
//             name.0 = "Elaina Hume".to_string();
//             break; // We don’t need to change any other names
//         }
//     }
// }

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (walk_player, follow_player_with_camera).chain());
    }
}
