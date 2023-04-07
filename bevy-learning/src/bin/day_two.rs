use std::f32::consts::PI;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::CursorIcon::Default;

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_systems((
            setup_cameras,
            setup
        ).on_startup())
        .add_system(move_camera)
        .run();
}

#[derive(Resource, Default)]
struct Movable {
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
    max_velocity: f32,
    entity: Option<Entity>,
}

#[derive(Resource, Default)]
struct Game {
    player: Movable,
    camera_angle: f32,
    camera_position: Vec3,
    camera_rotation: f32,
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
}

fn setup_cameras(mut commands: Commands, mut game: ResMut<Game>) {
    game.camera_should_focus = vec3(0., 0., 0.);
    game.camera_is_focus = game.camera_should_focus;
    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(5., 5., 5.)
                .looking_at(game.camera_is_focus, Vec3::Y),
            ..default()
        });
}

fn setup(mut commands: Commands, mut game: ResMut<Game>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(10.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 10.0, 4.0),
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            range: 30.0,
            ..default()
        },
        ..default()
    });
}

// control the game character
fn move_camera(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut moved = false;

    if keyboard_input.pressed(KeyCode::A) {
        game.camera_angle += 0.1;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::D) {
        game.camera_angle -= 0.1;
        moved = true;
    }
    if moved {
        println!("{}", Quat::from_rotation_y(game.camera_angle));
        for mut t in transforms.iter_mut() {
            let new_transform = Transform {
                translation: vec3(5., 5., 5.),
                rotation: Quat::from_rotation_y(game.camera_angle),
                ..default()
            }.looking_at(game.player.position, Vec3::Y);
            *t = new_transform;
        }
    }
}