mod geometry;

use std::f32::consts::PI;

use bevy::app::AppBuilder;
use bevy::pbr::AmbientLight;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::camera::Camera;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 2.0,
            })
            .add_startup_system(spawn_entities.system())
            .add_startup_system(generate_hex.system())
            // .add_startup_system(setup_ambient_light.system())
            .add_system(keyboard_controls.system())
            // .add_plugin(FrameTimeDiagnosticsPlugin::default())
            // .add_plugin(LogDiagnosticsPlugin::default())
            ;
    }
}

fn spawn_entities(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 4.0;
    camera.transform = Transform::from_matrix(Mat4::from_rotation_translation(
        Quat::from_rotation_ypr(-PI / 2.0, -PI / 2.0, 0.0).normalize(),
        Vec3::new(0.0, 20.0, 0.0),
    ));

    commands.spawn_bundle(camera);

    // commands.spawn_bundle(LightBundle {
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..Default::default()
    // });
}

// fn setup_ambient_light(mut ambient_light: ResMut<AmbientLight>) {
//     ambient_light.color = Color::WHITE;
// }

fn generate_hex(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(geometry::generate_hex_mesh());
    let color = Color::rgb(1.0, 1.0, 1.0);

    use rand::distributions::{Distribution, Uniform};

    let between = Uniform::from(-10.0..10.0);
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        add_hex(
            Vec3::new(between.sample(&mut rng), between.sample(&mut rng), between.sample(&mut rng)),
            color,
            mesh.clone(),
            &mut commands,
            &mut materials,
        );
    }
}

fn add_hex(
    position: Vec3,
    color: Color,
    mesh: Handle<Mesh>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh,
        material: materials.add(color.into()),
        transform: Transform::from_translation(position),
        ..Default::default()
    });
}

pub fn keyboard_controls(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    let (_, mut transform) = query.iter_mut().next().unwrap();
    let speed = 10.;
    let forward = Vec3::X;
    let right = Vec3::Z;
    let up = Vec3::Y;

    let mut pos = transform.translation.clone();
    if input.pressed(KeyCode::W) {
        pos += speed * forward * time.delta_seconds();
    } else if input.pressed(KeyCode::S) {
        pos -= speed * forward * time.delta_seconds();
    }
    if input.pressed(KeyCode::D) {
        pos += speed * right * time.delta_seconds();
    } else if input.pressed(KeyCode::A) {
        pos -= speed * right * time.delta_seconds();
    }
    if input.pressed(KeyCode::Q) {
        pos += speed * up * time.delta_seconds();
    } else if input.pressed(KeyCode::E) {
        pos -= speed * up * time.delta_seconds();
    }

    transform.translation = pos;
}
