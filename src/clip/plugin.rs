use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;

use crate::clip::output_target::OutputTarget;
use crate::clip::clip_rendering::{ClipLayerMaterial, update_clip_layer_blend};

pub struct PyreeClipPlugin;

impl Plugin for PyreeClipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(MaterialPlugin::<ClipLayerMaterial>::default())
            .add_startup_system(|mut commands: Commands, mut images: ResMut<Assets<Image>>| {
                let size = Extent3d { width: 1920, height: 1080, ..default() };
                commands.insert_resource(
                    OutputTarget::new(
                        size,
                        images,
                    )
                );
            })
            .add_system(update_clip_layer_blend)
        ;
    }
}