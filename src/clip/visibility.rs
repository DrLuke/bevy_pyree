//! Custom visibility for individual clips

use bevy::prelude::*;
use crate::clip::Clip;
use bevy_inspector_egui::Inspectable;

/// A counter to help with getting a unique layer number for each ClipVisibilityLayer
#[derive(Resource, Default)]
pub struct ClipVisibilityLayerAllocator(usize);

impl ClipVisibilityLayerAllocator {
    /// Get a layer number that hasn't been used yet
    pub fn get(&mut self) -> usize {
        // We return the current number, but have to increment it first
        let return_value = self.0;
        self.0 += 1;
        return_value
    }
}

/// Stores which clip visibility layer an entity belongs to.
/// The value must be the same as the parent clip entity's clip visibility layer.
/// This Component is then compared to the clip visibility layer of the rendering camera, and
/// sets the computed visibility to true if it is the same layer.
#[derive(Component, Inspectable)]
pub struct ClipVisiblityLayer(usize);

/// Recursively climb up the ancestry tree of an entity until a clip is reached and
/// return the clip visibility layer of that clip
fn get_parent_clip(
    target: Entity,
    clip_query: &Query<&Clip>,
    parents: &Query<&Parent>,
) -> Option<usize> {
    for parent in parents.iter_ancestors(target) {
        if let Ok(clip) = clip_query.get(parent) {
            return Some(clip.get_clip_visibility_layer());
        }
        if let Some(clip) = get_parent_clip(parent, clip_query, parents) {
            return Some(clip);
        }
    }

    None
}

/// Automatically add `ClipVisiblityLayer` to new cameras and entities with visibility
pub fn add_clip_visibility_layer(
    clip_query: Query<&Clip>,
    parents: Query<&Parent>,
    newly_added_entities: Query<Entity, (Added<ComputedVisibility>, Without<ClipVisiblityLayer>)>,
    newly_added_cameras: Query<Entity, (Added<Camera>, Without<ClipVisiblityLayer>)>,
    mut commands: Commands,
) {
    for new in newly_added_entities.iter() {
        if let Some(clip_visibility_layer) = get_parent_clip(new.clone(), &clip_query, &parents) {
            commands.entity(new).insert(ClipVisiblityLayer(clip_visibility_layer));
        }
    }
    for new in newly_added_cameras.iter() {
        if let Some(clip_visibility_layer) = get_parent_clip(new.clone(), &clip_query, &parents) {
            commands.entity(new).insert(ClipVisiblityLayer(clip_visibility_layer));
        }
    }
}