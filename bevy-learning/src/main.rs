//! A simple 3D scene with light shining over a cube sitting on a plane.
use std::ops::{Add, Div};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::WindowResized;

fn main() {
    App::new()
        .init_resource::<World>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_systems((
            move_cube,
            focus_camera
        ))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut world: ResMut<World>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    let sphere = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 1., sectors: 10, stacks: 10 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 80., 0.0),
        ..default()
    }).id();
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
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    world.movers.push(Mover {
        velocity: Default::default(),
        acceleration: Default::default(),
        location: vec3(0., 80., 0.0),
        entity: Some(sphere),
        mass: 1.,
    });

    world.wind = vec3(0.00001, 0., 0.);
    world.gravity = vec3(0.0, -0.001, 0.);
    world.camera_is_focus = vec3(-2.0, 2.5, 5.0);
    world.camera_should_focus = world.camera_is_focus;

}

#[derive(Resource, Default)]
struct Mover {
    velocity: Vec3,
    acceleration: Vec3,
    location: Vec3,
    mass: f32,
    entity: Option<Entity>,
}

impl Mover {
    pub fn apply_force(&mut self, force: &Vec3) {
        let f = force.div(self.mass);
        self.acceleration += f;
    }

    pub fn update(&mut self, [width, height]: [f32; 2]) {
        self.velocity = self.velocity.add(self.acceleration);
        self.location += self.velocity;

        if self.location.x > width {
            self.location.x = width;
            self.velocity.x *= -1.;
        } else if self.location.x < 0. {
            self.velocity.x *= -1.;
            self.location.x = 0.;
        }

        if self.location.y > height {
            self.velocity.y *= -1.;
            self.location.y = height;
        } else if self.location.y < 0. {
            self.velocity.y *= -1.;
            self.location.y = 0.;
        }
    }
}

#[derive(Resource, Default)]
struct World {
    movers: Vec<Mover>,
    wind: Vec3,
    gravity: Vec3,
    camera_is_focus: Vec3,
    camera_should_focus: Vec3,
    bounds: [f32; 2],

}

fn move_cube(
    mut commands: Commands,
    mut world: ResMut<World>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform>,
) {
    let wind = world.wind.clone();
    let gravity = world.gravity.clone();
    let bounds = world.bounds.clone();
    let mut movers = &mut world.movers;
    for mover in movers.iter_mut() {
        mover.apply_force(&wind);
        mover.apply_force(&gravity);
        mover.update(bounds);
        if let Some(entity) = mover.entity {
            if let Ok(mut entity_transform) = transforms.get_mut(entity) {
                entity_transform.translation = mover.location.clone();
            }
        }
    }
}


fn focus_camera(
    time: Res<Time>,
    mut world: ResMut<World>,
    mut transforms: ParamSet<(Query<&mut Transform, With<Camera3d>>, Query<&Transform>)>
) {
    let mover = world.movers.first().unwrap().entity.unwrap();
    const SPEED: f32 = 2.0;
    // if there is both a player and a bonus, target the mid-point of them
    if let Ok(player_transform) = transforms.p1().get(mover) {
        world.camera_should_focus = player_transform.translation;
    }
    // calculate the camera motion based on the difference between where the camera is looking
    // and where it should be looking; the greater the distance, the faster the motion;
    // smooth out the camera movement using the frame time
    let mut camera_motion = world.camera_should_focus - world.camera_is_focus;
    if camera_motion.length() > 0.2 {
        camera_motion *= SPEED * time.delta_seconds();
        // set the new camera's actual focus
        world.camera_is_focus += camera_motion;
    }
    // look at that new camera's actual focus
    for mut transform in transforms.p0().iter_mut() {
        *transform = transform.looking_at(world.camera_is_focus, Vec3::Y);
    }

}


/// This system shows how to respond to a window being resized.
/// Whenever the window is resized, the text will update with the new resolution.
fn on_resize_system(
    mut world: ResMut<World>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.iter() {
        world.bounds = [e.width, e.height];
    }
}
