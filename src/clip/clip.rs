use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use crate::clip::visibility::ClipVisibilityLayerAllocator;
use bevy_inspector_egui::Inspectable;


/// A clip is something that contains a RenderTarget and some scene or video that is rendered
/// into that RenderTarget. It's the first step of pixel generation in the VJ chain.
/// The Clip is responsible for rendering stuff into the RenderTarget, preferrably using Layers
#[derive(Component, Inspectable)]
pub struct Clip {
    /// Display name for this clip
    display_name: String,
    /// The image this clip will render into
    pub render_target: Handle<Image>,
    /// True if the clip is currently being used/displayed.
    /// Can be used by clip creators to, for example, stop expensive computations while the clip is not used.
    active: bool,
    /// The clip visibility layer this clip is on.
    /// This should be unique across all clips and is used to determine which entities will be rendered for this clip.
    clip_visibility_layer: usize,
}

impl Clip {
    pub fn new(
        display_name: String,
        images: &mut ResMut<Assets<Image>>,
        size: Extent3d,
        clip_visibility_layer_allocator: &mut ResMut<ClipVisibilityLayerAllocator>,
    ) -> Self {
        // This is the texture that will be rendered to.
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

        let image_handle = images.add(image);

        Self {
            display_name,
            render_target: image_handle,
            active: true,
            clip_visibility_layer: clip_visibility_layer_allocator.get()
        }
    }

    pub fn from_image(
        display_name: String,
        image: Handle<Image>,
        clip_visibility_layer_allocator: &mut ResMut<ClipVisibilityLayerAllocator>,
    ) -> Self {
        Self {
            display_name,
            render_target: image,
            active: true,
            clip_visibility_layer: clip_visibility_layer_allocator.get()
        }
    }

    pub fn get_display_name(&self) -> String {
        self.display_name.clone()
    }

    pub fn is_active(&self) -> bool { self.active }
    pub fn set_active(&mut self, active: bool) { self.active = active }
    pub fn activate(&mut self) { self.set_active(true) }
    pub fn deactivate(&mut self) { self.set_active(false) }
    pub fn get_clip_visibility_layer(&self) -> usize { self.clip_visibility_layer }
}
