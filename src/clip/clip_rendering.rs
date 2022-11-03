use bevy::prelude::*;
use bevy::{
    reflect::TypeUuid,
    render::{
        render_resource::{AsBindGroup, ShaderRef},
    },
};
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use crate::clip::{ClipLayer};

/// A texture a layer render's into
#[derive(Component)]
pub struct ClipLayerRenderTarget {
    pub render_target: Handle<Image>,
}

impl ClipLayerRenderTarget {
    pub fn new(
        images: &mut ResMut<Assets<Image>>,
        size: Extent3d,
    ) -> Self {
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
            },
            ..default()
        };
        image.resize(size);

        Self {
            render_target: images.add(image)
        }
    }
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "b9dc231d-b94d-4cdf-9a7e-b527f6720a60"]
pub struct ClipLayerMaterial {
    #[uniform(0)]
    pub blend: f32,
    /// The output from a previous layer, if any
    #[texture(1)]
    #[sampler(2)]
    pub previous_rt: Option<Handle<Image>>,
    /// The selected clip that is to be blended into the previous frame
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

/// Whenever a clip layer is added/removed, we need to refresh the entire render target chain
pub fn update_render_target_chain(
    mut clip_layer_query: Query<(&ClipLayer, &ClipLayerRenderTarget, &Handle<ClipLayerMaterial>)>,
    clip_layer_added_query: Query<&ClipLayer, Added<ClipLayer>>,
    mut materials: ResMut<Assets<ClipLayerMaterial>>,
) {
    if let Some(_) = clip_layer_added_query.iter().next() {
        let mut clip_layers: Vec<(&ClipLayer, &ClipLayerRenderTarget, &Handle<ClipLayerMaterial>)> = clip_layer_query.iter().collect();
        clip_layers.sort_by(|a, b| a.0.layer.cmp(&b.0.layer));
        let mut prev_handle = None;
        for (cl, clt, material_handle) in clip_layers.iter_mut() {
            if let Some(material) = materials.get_mut(material_handle) {
                material.previous_rt = prev_handle;
            }
            prev_handle = Some(clt.render_target.clone());
        }
    }
}
