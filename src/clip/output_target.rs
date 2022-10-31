use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};

/// Resource that contains the output image into which the multiple layers will render into
/// The render target contained within is what is finally rendered onto the screen
#[derive(Component)]
pub struct OutputTarget {
    pub render_target: Handle<Image>,
}

impl OutputTarget {
    pub fn new(size: Extent3d, mut images: ResMut<Assets<Image>>) -> Self {
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

        OutputTarget {
            render_target: image_handle
        }
    }
}