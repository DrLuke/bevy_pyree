use bevy::prelude::*;
use bevy::render::camera::{Projection, ScalingMode};
use crate::clip::clip_rendering::ClipLayerMaterial;
use crate::clip::ClipLayer;

/// This bundle represents a single clip layer and everything required to render it
#[derive(Bundle)]
pub struct ClipLayerBundle {
    pub clip_layer: ClipLayer,

    #[bundle]
    clip_layer_quad: MaterialMeshBundle<ClipLayerMaterial>,
}

impl ClipLayerBundle {
    pub fn new(
        layer: u8,
        mut materials: ResMut<Assets<ClipLayerMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
        output_render_target: Handle<Image>,
    ) -> Self {
        let render_mesh = MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 2.0 })),
            material: materials.add(ClipLayerMaterial {
                blend: 0.0,
                output_rt: output_render_target,
                clip_rt: None,
            }),
            ..default()
        };

        ClipLayerBundle {
            clip_layer: ClipLayer::new(layer),
            clip_layer_quad: render_mesh
        }
    }
}