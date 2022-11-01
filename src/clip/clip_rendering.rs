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
    #[uniform(0)]
    pub blend: f32,
    #[texture(1)]
    #[sampler(2)]
    pub output_rt: Handle<Image>,
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
    mut clip_layer_query: Query<(&ClipLayer, &Handle<ClipLayerMaterial>), Changed<ClipLayer>>,
    mut materials: ResMut<Assets<ClipLayerMaterial>>,
)
{
    for (clip_layer, material_handle) in clip_layer_query.iter_mut() {
        if let Some(material) = materials.get_mut(material_handle) {
            if material.blend != clip_layer.blend {
                material.blend = clip_layer.blend;
            }
            let active_rt = clip_layer.get_render_targets()[clip_layer.get_active_clip() as usize].clone();
            if material.clip_rt != active_rt {
                material.clip_rt = active_rt;
            }
        } else {
            // TODO: print error?
        }
    }
}