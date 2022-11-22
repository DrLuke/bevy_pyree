use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use bevy_inspector_egui::RegisterInspectable;

use crate::clip::clip_rendering::{ClipLayerMaterial, update_clip_layer_blend};
use crate::clip::{Clip, ClipLayerLastRenderTarget, update_render_target_chain};
use crate::clip::visibility::{add_clip_visibility_layer, ClipVisibilityLayerAllocator, ClipVisiblityLayer};

pub struct PyreeClipPlugin;

impl Plugin for PyreeClipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(MaterialPlugin::<ClipLayerMaterial>::default())
            .insert_resource(ClipVisibilityLayerAllocator::default())
            .add_startup_system(|mut commands: Commands, mut images: ResMut<Assets<Image>>| {
                commands.insert_resource(ClipLayerLastRenderTarget { render_target: None })
            })
            .add_system(update_clip_layer_blend)
            .add_system(update_render_target_chain)
            .add_system(add_clip_visibility_layer)

            .register_inspectable::<Clip>()
            .register_inspectable::<ClipVisiblityLayer>()
        ;
    }
}