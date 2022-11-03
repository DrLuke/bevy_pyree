use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;

use crate::clip::clip_rendering::{ClipLayerMaterial, update_clip_layer_blend};
use crate::clip::update_render_target_chain;

pub struct PyreeClipPlugin;

impl Plugin for PyreeClipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(MaterialPlugin::<ClipLayerMaterial>::default())
            .add_startup_system(|mut commands: Commands, mut images: ResMut<Assets<Image>>| {

            })
            .add_system(update_clip_layer_blend)
            .add_system(update_render_target_chain)
        ;
    }
}