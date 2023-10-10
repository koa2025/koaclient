use bevy::{
    prelude::*,
};
use bevy::math::{vec3, Vec3A};
use bevy::pbr::{MaterialMeshBundle, MaterialPipeline, MaterialPipelineKey};
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::mesh::{MeshVertexAttribute, MeshVertexBufferLayout};
use bevy::render::primitives::{Aabb, Sphere};
use bevy::render::render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError, VertexFormat};
use bevy_sprite3d::{Sprite3d, Sprite3dParams, Sprite3dPlugin};
use crate::{CustomMaterial, MyAssets, GameState, MainCamera, MyMaterials};
use crate::tools::{CameraController, parse_scene, SceneHandle, SceneViewerPlugin};
use rand::Rng; // 引入随机数生成器

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((Sprite3dPlugin))
            .add_plugins((SceneViewerPlugin))
            .add_systems(OnEnter(GameState::Init), setup)
            .add_systems(PreUpdate, setup_scene_after_load.run_if(in_state(GameState::Playing)))
        // .add_systems(Update, material.run_if(in_state(GameState::Playing)))
        ;
    }
}

fn setup(
    mut commands: Commands,
    mut cameras: Query<&Transform, With<MainCamera>>,
    asset_server: Res<AssetServer>,
    my_assets: Res<MyAssets>,
    mut game_state: ResMut<NextState<GameState>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut scene_handle: ResMut<SceneHandle>,
    // mut scene_spawner: ResMut<SceneSpawner>,
    mut sprite_params: Sprite3dParams,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,

) {
    commands.spawn(Sprite3d {
        image: my_assets.tree.clone(),
        pixels_per_metre: 400.,
        // partial_alpha: true,
        unlit: true,
        transform: Transform::from_scale(vec3(10.,10.,0.)),
        // pivot: Some(Vec2::new(0.5, 0.5)),
        ..default()
    }.bundle(&mut sprite_params)).insert(custom_materials.add(CustomMaterial { color: Color::GREEN }));

    let scene_path = "models/fmj.gltf";
    let (file_path, scene_index) = parse_scene(scene_path.into());
    commands.insert_resource(SceneHandle::new(asset_server.load(file_path), scene_index));
    // let camera = cameras.single_mut();
    //
    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         shadows_enabled: false,
    //         ..default()
    //     },
    //     transform: Transform::from_rotation(camera.rotation.clone()),
    //     // // This is a relatively small scene, so use tighter shadow
    //     // // cascade bounds than the default for better quality.
    //     // // We also adjusted the shadow map to be larger since we're
    //     // // only using a single cascade.
    //     // cascade_shadow_config: CascadeShadowConfigBuilder {
    //     //     num_cascades: 1,
    //     //     maximum_distance: 1.6,
    //     //     ..default()
    //     // }
    //     //     .into(),
    //     ..default()
    // });
    //
    // commands.spawn(SceneBundle {
    //     scene: game_assets.scene0.clone(),
    //     ..default()
    // });

    // commands.insert_resource(MyMaterials {
    //     grass: materials.add(CustomMaterial {
    //         color: Color::GREEN,
    //     })
    // });

    // cube
    // commands.spawn(MaterialMeshBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     material:materials.add(CustomMaterial {
    //         color: Color::GREEN,
    //     }),
    //     ..default()
    // });
    game_state.set(GameState::Playing);
}

fn setup_scene_after_load(
    mut commands: Commands,
    mut setup: Local<bool>,
    mut scene_handle: ResMut<SceneHandle>,
    asset_server: Res<AssetServer>,
    meshes_query: Query<(&GlobalTransform, Option<&Aabb>), With<Handle<Mesh>>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(Entity, &Name, &GlobalTransform, &Handle<Mesh>, &Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
    if scene_handle.is_loaded && !*setup {
        *setup = true;
        // Find an approximate bounding box of the scene from its meshes
        if meshes_query.iter().any(|(_, maybe_aabb)| maybe_aabb.is_none()) {
            return;
        }

        let mut min = Vec3A::splat(f32::MAX);
        let mut max = Vec3A::splat(f32::MIN);
        for (transform, maybe_aabb) in &meshes_query {
            let aabb = maybe_aabb.unwrap();
            // If the Aabb had not been rotated, applying the non-uniform scale would produce the
            // correct bounds. However, it could very well be rotated and so we first convert to
            // a Sphere, and then back to an Aabb to find the conservative min and max points.
            let sphere = Sphere {
                center: Vec3A::from(transform.transform_point(Vec3::from(aabb.center))),
                radius: transform.radius_vec3a(aabb.half_extents),
            };
            let aabb = Aabb::from(sphere);
            min = min.min(aabb.min());
            max = max.max(aabb.max());
        }

        let size = (max - min).length();
        let aabb = Aabb::from_min_max(Vec3::from(min), Vec3::from(max));

        info!("Spawning a controllable 3D perspective camera");
        let mut projection = PerspectiveProjection::default();
        projection.far = projection.far.max(size * 10.0);

        let camera_controller = CameraController::default();

        // Display the controls of the scene viewer
        info!("{}", camera_controller);
        info!("{}", *scene_handle);

        // commands.spawn((
        //     Camera3dBundle {
        //         projection: projection.into(),
        //         transform: Transform::from_translation(
        //             Vec3::from(aabb.center) + size * Vec3::new(0.5, 0.25, 0.5),
        //         )
        //             .looking_at(Vec3::from(aabb.center), Vec3::Y),
        //         camera: Camera {
        //             is_active: false,
        //             ..default()
        //         },
        //         ..default()
        //     },
        //     EnvironmentMapLight {
        //         diffuse_map: asset_server
        //             .load("assets/environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
        //         specular_map: asset_server
        //             .load("assets/environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        //     },
        //     camera_controller,
        // ));

        // Spawn a default light if the scene does not have one
        if !scene_handle.has_light {
            info!("Spawning a directional light");
            commands.spawn(DirectionalLightBundle {
                directional_light: DirectionalLight {
                    shadows_enabled: false,
                    ..default()
                },
                ..default()
            });

            scene_handle.has_light = true;
        }

        let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
        for i in 0..10 {
            let x = rand::thread_rng().gen_range(-2.0..=2.0);
            let y = 2.;
            let z = rand::thread_rng().gen_range(-2.0..=2.0);
            let mut transform = Transform::from_xyz(x, y, z);
            transform.scale = vec3(0.1, 0.2, 0.);
            commands.spawn(MaterialMeshBundle {
                mesh: mesh.clone(),
                // material: materials.add(StandardMaterial {
                //     base_color: Color::WHITE,
                //     ..default()
                // }),
                material: custom_materials.add(CustomMaterial { color: Color::GREEN }),
                transform,
                ..default()
            });
        }


        for (entity, name, global_transform, mesh, material_handle) in query.iter_mut() {
            // if name.contains("SharedMesh") {
            //     // let material = materials.get_mut(material_handle).unwrap();
            //     // material.base_color = Color::rgb(0.0, 0.0, 1.0);
            //     info!("SharedMesh base_color : {:?}", entity);
            //     commands.entity(entity).despawn();
            //     // commands.entity(entity).remove::<Handle<StandardMaterial>>();
            //     //添加材质无效
            //     // commands.entity(entity).insert(materials.add(StandardMaterial {
            //     //     base_color: Color::RED,
            //     //     ..default()
            //     // }));
            //     // commands.entity(entity).insert(custom_materials.add(CustomMaterial {
            //     //     color: Color::GREEN,
            //     // }));
            // }
            if name.contains("PixelMesh") {
                // let material = materials.get_mut(material_handle).unwrap();
                // material.base_color = Color::rgb(0.0, 0.0, 1.0);
                info!("PixelMesh: {:?}", entity);
                commands.entity(entity).remove::<Handle<StandardMaterial>>();
                //添加材质无效
                // commands.entity(entity).insert(materials.add(StandardMaterial {
                //     base_color: Color::RED,
                //     ..default()
                // }));
                commands.entity(entity).insert(custom_materials.add(CustomMaterial {
                    color: Color::GREEN,
                }));
            }
        }
    }
}

fn material(
    mut commands: Commands,
    mut query: Query<(Entity, &Name, &GlobalTransform, &Handle<Mesh>, &Handle<StandardMaterial>)>,
    mut my_materials: Res<MyMaterials>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, name, global_transform, mesh, material_handle) in query.iter_mut() {
        if name.contains("SharedMesh") {
            let material = materials.get_mut(material_handle).unwrap();
            material.base_color = Color::rgb(0.8, 0.7, 0.6);
            info!("SharedMesh base_color : {:?}", entity);
            // commands.entity(entity).remove::<Handle<StandardMaterial>>();
            // // commands.entity(entity).insert(my_materials.grass.clone());
            // let material = materials.add(StandardMaterial {
            //     base_color: Color::BLUE,
            //     ..default()
            // });
            // commands.entity(entity).insert(material);
            // let mut transform = Transform::from_translation(global_transform.translation());
            // transform.rotation.x = 1.5;
            // transform.scale.x = 0.2;
            // commands.spawn(MaterialMeshBundle {
            //     // mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            //     mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0, subdivisions: 8 })),
            //     transform,
            //     material: my_materials.grass.clone(),
            //     // material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            //     ..default()
            // });

            // commands.entity(entity).insert(my_materials.grass.clone());
        }
    }
}