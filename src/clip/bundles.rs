use bevy::prelude::*;
use bevy::render::camera::{Projection, RenderTarget, ScalingMode};
use bevy::render::render_resource::Extent3d;
use bevy::render::view::RenderLayers;
use crate::clip::clip_rendering::{ClipLayerMaterial, ClipLayerRenderTarget};
use crate::clip::ClipLayer;

/// This bundle represents a single clip layer and everything required to render it
#[derive(Bundle)]
pub struct ClipLayerBundle {
    pub clip_layer: ClipLayer,
    clip_layer_render_target: ClipLayerRenderTarget,

    #[bundle]
    clip_layer_quad: MaterialMeshBundle<ClipLayerMaterial>,
}

impl ClipLayerBundle {
    pub fn new(
        layer: u8,
        materials: &mut ResMut<Assets<ClipLayerMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
        images: &mut ResMut<Assets<Image>>,
    ) -> Self {
        let render_mesh = MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 2.0 })),
            material: materials.add(ClipLayerMaterial {
                blend: 0.5,
                previous_rt: None,
                clip_rt: None,
            }),
            ..default()
        };

        ClipLayerBundle {
            clip_layer: ClipLayer::new(layer),
            clip_layer_render_target: ClipLayerRenderTarget::new(
                images,
                Extent3d { width: 1920, height: 1080, ..default() },
            ),
            clip_layer_quad: render_mesh,
        }
    }
}

/// Spawn a clip layer bundle with the correct children cameras
pub fn spawn_clip_layer_bundle(
    commands: &mut Commands,
    clip_layer_bundle: ClipLayerBundle,
    render_layer: u8, // Temporary until a better solution is found
) -> Entity {
    let layer = clip_layer_bundle.clip_layer.layer;
    let render_target = clip_layer_bundle.clip_layer_render_target.render_target.clone();
    commands.spawn_bundle(
        clip_layer_bundle
    )
        .insert(RenderLayers::layer(render_layer))
        .with_children(|child_builder| {
            child_builder.spawn_bundle(Camera3dBundle {
                camera: Camera {
                    priority: 100 + layer as isize,
                    target: RenderTarget::Image(render_target),
                    ..default()
                },
                projection: Projection::Orthographic(OrthographicProjection {
                    scaling_mode: ScalingMode::None,
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)).looking_at(Vec3::default(), Vec3::Z),
                ..default()
            })
                .insert(RenderLayers::layer(render_layer));
        })
        .id()
}