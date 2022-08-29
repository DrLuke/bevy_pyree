use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::render::view::RenderLayers;


/// A clip is something that contains a RenderTarget and some scene or video that is rendered
/// into that RenderTarget. It's the first step of pixel generation in the VJ chain.
/// The Clip is responsible for rendering stuff into the RenderTarget, preferrably using Layers
#[derive(Component)]
pub struct Clip {
    /// The image this clip will render into
    pub render_target: Handle<Image>,
    /// Each clip gets it's own render layer
    pub clip_layer: usize,
    /// Whether this clip is currently active and running
    active: bool,
}

impl Clip {
    pub fn new(
        clip_layer: usize,
        mut images: ResMut<Assets<Image>>,
        size: Extent3d,
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
            render_target: image_handle,
            clip_layer,
            active: true
        }
    }
}

/// An entity that belongs to a certain clip with
#[derive(Component)]
struct ClipLayer(pub usize);