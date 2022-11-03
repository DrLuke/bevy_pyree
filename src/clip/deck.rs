/// Deck accepts clips as input and renders them to the screen

use bevy::prelude::*;
use crate::clip::Clip;
use bevy::{
    reflect::TypeUuid,
    render::{
        camera::ScalingMode,
        camera::Projection,
        render_resource::{AsBindGroup, ShaderRef},
        renderer::RenderQueue,
    },
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_resource::*,
        Extract, RenderApp, RenderStage,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, RenderMaterials2d},
    window::PresentMode,
    pbr::RenderMaterials,
};
use bevy::render::view::RenderLayers;

/// Simple Deck with 2 slots and crossfader
#[derive(Default, Component, Clone, Copy)]
pub struct Deck2 {
    pub slot_a: Option<Entity>,
    pub slot_b: Option<Entity>,
    pub crossfade: f32,
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "42782bd2-0460-4595-b066-0a6db958199e"]
pub struct Deck2Material {
    #[uniform(0)]
    pub fade_ab: f32,
    #[texture(1)]
    #[sampler(2)]
    pub image_a: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    pub image_b: Option<Handle<Image>>,
}

impl Material for Deck2Material {
    fn fragment_shader() -> ShaderRef {
        "shader.wgsl".into()
    }
}

#[derive(Component)]
pub struct DeckRenderer;

pub fn setup_deck2(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<Deck2Material>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let render_mesh = MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 2.0 })),
        material: materials.add(Deck2Material {
            fade_ab: 0.0,
            image_a: None,
            image_b: None,
        }),
        ..default()
    };

    commands.spawn_bundle(Camera3dBundle {
        camera: Camera {
            priority: 10000000,
            ..default()
        },
        projection: Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::None,
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)).looking_at(Vec3::default(), Vec3::Z),
        ..default()
    }).insert(RenderLayers::layer(10));

    commands.spawn_bundle(render_mesh).insert(DeckRenderer).insert(RenderLayers::layer(10));
}


pub struct ExtractedCrossfade(f32);

impl ExtractResource for ExtractedCrossfade {
    type Source = Deck2;

    fn extract_resource(time: &Self::Source) -> Self {
        ExtractedCrossfade {
            0: time.crossfade,
        }
    }
}

pub fn extract_deck2(
    mut commands: Commands,
    query: Extract<Query<(Entity, &Handle<Deck2Material>)>>,
) {
    for (entity, handle) in query.iter() {
        commands.get_or_spawn(entity)
            .insert(handle.clone());
    }
}

#[derive(Clone, ShaderType)]
struct Deck2MaterialUniformData {
    crossfade: f32,
}

pub fn prepare_deck2(
    materials: Res<RenderMaterials<Deck2Material>>,
    query: Query<&Handle<Deck2Material>>,
    crossfade: Res<ExtractedCrossfade>,
    render_queue: Res<RenderQueue>,
) {
    for handle in query.iter() {
        if let Some(material) = materials.get(handle) {
            let binding = &material.bindings[4];
            if let OwnedBindingResource::Buffer(cur_buffer) = binding {
                let mut buffer = encase::UniformBuffer::new(Vec::new());
                buffer
                    .write(&Deck2MaterialUniformData{
                        crossfade: crossfade.0
                    })
                    .unwrap();
                render_queue.write_buffer(cur_buffer, 0, buffer.as_ref());
            }
        }
    }
}