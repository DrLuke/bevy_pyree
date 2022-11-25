//! Custom visibility for individual clips

use bevy::prelude::*;
use crate::clip::Clip;
use bevy_inspector_egui::Inspectable;

/// Stores the clip a camera belongs to. This is used to determine the visibility of a clip.
#[derive(Component, Inspectable)]
pub struct ClipCamera(Entity);

fn find_parent_clip(
    target: Entity,
    parent_query: &Query<&Parent>,
    clip_query: &Query<Entity, With<Clip>>,
) -> Option<Entity> {
    for parent in parent_query.iter_ancestors(target) {
        if let Ok(clip_entity) = clip_query.get(parent) {
            return Some(clip_entity);
        }
        if let Some(clip_entity) = find_parent_clip(parent, parent_query, clip_query) {
            return Some(clip_entity);
        }
    }

    None
}

/// Automatically add `ClipCamera` to new cameras that are children of a clip
pub fn add_clip_camera_component(
    clip_query: Query<Entity, With<Clip>>,
    parents: Query<&Parent>,
    newly_added_cameras: Query<Entity, (Added<Camera>, Without<ClipCamera>)>,
    mut commands: Commands,
) {
    for camera_entity in newly_added_cameras.iter() {
        let clip_entity_maybe = find_parent_clip(camera_entity, &parents, &clip_query);
        if let Some(clip_entity) = clip_entity_maybe {
            commands.entity(camera_entity).insert(ClipCamera(clip_entity));
        }
    }
}