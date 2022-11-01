use bevy::prelude::*;
use bevy::{
    reflect::TypeUuid,
    render::{
        render_resource::{AsBindGroup, ShaderRef},
    },
};
use crate::clip::{ClipLayer, OutputTarget};

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "b9dc231d-b94d-4cdf-9a7e-b527f6720a60"]
pub struct ClipLayerMaterial {
    /// When updating this material, we need to know which entity it belongs to
    pub clip_layer: Entity,
    #[uniform(0)]
    pub blend: f32,
    #[texture(1)]
    #[sampler(2)]
    pub output_rt: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    pub clip_rt: Option<Handle<Image>>,
}

impl Material for ClipLayerMaterial {
    fn fragment_shader() -> ShaderRef {
        "layer_render.wgsl".into()
    }
}

/// Update the blend value and render targets on material if it changed
pub fn update_clip_layer_blend(
    clip_layer_query: Query<(Entity, &ClipLayer), Changed<ClipLayer>>,
    mut materials: ResMut<Assets<ClipLayerMaterial>>,
    output_target: Res<OutputTarget>
)
{
     for (_, mut material) in materials.iter_mut() {
         let entity = material.clip_layer;
         if let Ok((_, clip_layer)) = clip_layer_query.get(entity) {
             if material.blend != clip_layer.blend {
                 material.blend = clip_layer.blend;
             }
         }
     }
}