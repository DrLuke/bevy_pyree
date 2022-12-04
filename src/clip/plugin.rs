use bevy::prelude::*;
use bevy::render::camera::camera_system;
use bevy::render::render_resource::Extent3d;
use bevy::render::view::VisibilitySystems::{CheckVisibility, VisibilityPropagate};
use bevy_inspector_egui::RegisterInspectable;
use bevy::render::view::visibility::*;
use bevy::transform::TransformSystem;
use bevy_rosc::{BevyRoscPlugin, method_dispatcher_system};

use crate::clip::clip_rendering::{ClipLayerMaterial, update_clip_layer_blend};
use crate::clip::{Clip, ClipLayer, ClipLayerLastRenderTarget, update_render_target_chain};
use crate::clip::visibility::{add_clip_camera_component_system, clip_visibility_system, ClipEntity};

pub struct PyreeClipPlugin;

impl Plugin for PyreeClipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(MaterialPlugin::<ClipLayerMaterial>::default())
            .add_startup_system(|mut commands: Commands, mut images: ResMut<Assets<Image>>| {
                commands.insert_resource(ClipLayerLastRenderTarget { render_target: None })
            })
            .add_system(update_clip_layer_blend)
            .add_system(update_render_target_chain)
            .add_system(add_clip_camera_component_system)
            //.add_system(add_visibility_to_clip_system)

            // Compute Clip visibility before visibility is propagated to child entities
            .add_system_to_stage(
                CoreStage::PostUpdate,
                clip_visibility_system.after(CheckVisibility),
            )

            .add_system(method_dispatcher_system::<ClipLayer>)

            .register_inspectable::<Clip>()
            .register_inspectable::<ClipEntity>()
        ;
    }
}