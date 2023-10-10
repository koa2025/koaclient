pub mod plugins;
pub mod tools;

use bevy::prelude::{Color, Component, Handle, Image, Material, Mesh, Resource, Scene, States};
use bevy_asset_loader::prelude::*;
use bevy::asset::AssetServer;
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError};

#[derive(States, Hash, Clone, PartialEq, Eq, Debug, Default)]
pub enum GameState {
    #[default]
    Loading,
    Init,
    Playing,
}


#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(path = "textures/background.png")]
    pub background: Handle<Image>,

    #[asset(path = "textures/tree.png")]
    pub tree: Handle<Image>,

    // #[asset(path = "models/fmj.gltf#Scene0")]
    // pub scene0: Handle<Scene>,
}

#[derive(Resource)]
pub struct MyMaterials {
    pub grass: Handle<CustomMaterial>,
}

#[derive(Component)]
pub struct MainCamera;

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, Debug, Clone, TypeUuid, TypePath)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/custom_vertex_attribute.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_vertex_attribute.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            // Mesh::ATTRIBUTE_COLOR.at_shader_location(1),
            // ATTRIBUTE_BLEND_COLOR.at_shader_location(1),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

