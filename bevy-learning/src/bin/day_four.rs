use std::f32::consts::PI;
use std::ops::Add;
use bevy::math::vec3;
use bevy::prelude::*;

const RESET_FOCUS: [f32; 3] = [
    -2.0,
    2.5,
    5.0
];

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_systems(
            (
                move_player,
                focus_camera
            )
        ).run();
}

#[derive(Resource, Default)]
struct Player {
    entity: Option<Entity>,
    nose: Option<Entity>,
    position: Vec3,
    direction: f32,
}

#[derive(Resource, Default)]
struct Game {
    player: Player,
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
    camera_position: Vec3,
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut game: ResMut<Game>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    let e = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    }).id();
    game.player.entity = Some(e);
    let e = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.3, 0.5, 0.0),
        ..default()
    }).id();
    game.player.nose = Some(e);
    game.player.position = vec3(0.0, 0.5, 0.0);
    game.player.direction = 0.;
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    game.camera_position = vec3(-2.0, 2.5, 5.0);
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// control the game character
fn move_player(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
) {
    let mut moved = false;
    let move_amount = 0.01;

    if keyboard_input.pressed(KeyCode::Up) {
        game.player.position.z -= move_amount;
        game.player.direction -= -PI / 2.;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        game.player.position.z += move_amount;
        game.player.direction -= PI / 2.;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        game.player.position.x += move_amount;
        game.player.direction = 0.;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        game.player.position.x -= move_amount;
        game.player.direction = PI;
        moved = true;
    }

    if moved {
        *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
            translation: game.player.position,
            rotation: Quat::from_rotation_y(game.player.direction),
            ..default()
        };
        *transforms.get_mut(game.player.nose.unwrap()).unwrap() = Transform {
            translation: game.player.position + vec3(0.3, 0., 0.),
            rotation: Quat::from_rotation_y(game.player.direction),
            ..default()
        };
    }
}


// change the focus of the camera
fn focus_camera(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut transforms: ParamSet<(Query<&mut Transform, With<Camera3d>>, Query<&Transform>)>,
) {
    const SPEED: f32 = 2.0;
    if let Some(player_entity) = game.player.entity {
        if let Ok(player_transform) = transforms.p1().get(player_entity) {
            game.camera_should_focus = player_transform.translation;
        }
        // otherwise, target the middle
    } else {
        game.camera_should_focus = Vec3::from(RESET_FOCUS);
    }
    // calculate the camera motion based on the difference between where the camera is looking
    // and where it should be looking; the greater the distance, the faster the motion;
    // smooth out the camera movement using the frame time
    let mut camera_motion = game.camera_should_focus - game.camera_is_focus;
    if camera_motion.length() > 0.2 {
        camera_motion *= SPEED * time.delta_seconds();
        // set the new camera's actual focus
        game.camera_is_focus += camera_motion;
    }
    for mut transform in transforms.p0().iter_mut() {
        let mut p = transform.translation;
        let mut camera_speed = game.player.position - p;
        if camera_speed.length() > 4.0 {
            camera_speed *= SPEED * time.delta_seconds();
            game.camera_position += camera_speed;
        }
        // look at that new camera's actual focus
        *transform = Transform::from_translation(game.camera_position).looking_at(game.camera_is_focus, Vec3::Y);
        //transform.looking_at(game.camera_is_focus, Vec3::Y);
    }
}