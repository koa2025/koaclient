use bevy::{
    render::{
        camera::{Projection},
    },
    prelude::*,
};
use bevy::window::{WindowMode};
use mia::{CustomMaterial, GameState, MainCamera, MyMaterials};
use mia::plugins::{GamePlugin, InspectPlugin, LoadPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }), MaterialPlugin::<CustomMaterial>::default()
        ))
        .add_plugins((
            LoadPlugin,
            InspectPlugin,
            GamePlugin,
        ))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // Calculate the camera position and direction to look at (0, 0, 0)

    let camera_position = Vec3::new(0., 15., 34.); // Adjust as needed
    let look_at_point = Vec3::ZERO;
    let up_direction = Vec3::new(0.0, 1., 0.0); // Y-axis as up

    // Create the camera with a 30-degree field of view
    commands.spawn((Camera3dBundle {
        transform: Transform::from_translation(camera_position)
            .looking_at(look_at_point, up_direction),
        projection: Projection::Perspective(PerspectiveProjection {
            fov: f32::to_radians(30.0),
            ..default()
        }),
        ..default()
    }, MainCamera));

    //
    //
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });
}


