use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};

/// A clip is something that contains a RenderTarget and some scene or video that is rendered
/// into that RenderTarget. It's the first step of pixel generation in the VJ chain.
/// The Clip is responsible for rendering stuff into the RenderTarget, preferrably using Layers
#[derive(Component)]
pub struct Clip {
    /// The image this clip will render into
    pub render_target: Handle<Image>,
}

impl Clip {
    pub fn new(
        mut images: ResMut<Assets<Image>>,
        size: Extent3d
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

        Self{
            render_target: image_handle
        }
    }
}