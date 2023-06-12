//! Custom visibility for individual clips

use bevy::prelude::*;
use bevy::render::view::VisibleEntities;
use crate::clip::Clip;
use bevy_inspector_egui::Inspectable;

/// Stores the clip an entity belongs to.
/// This is used to determine whether or not an entity should be visible for a given camera.
//#[derive(Component, Inspectable)]
#[derive(Component)]
pub struct ClipEntity(pub Entity);

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

/// Automatically add `ClipEntity` to new cameras and entities that are children of a clip
pub fn add_clip_camera_component_system(
    clip_query: Query<Entity, With<Clip>>,
    parents: Query<&Parent>,
    newly_added_cameras: Query<Entity, (Added<Camera>, Without<ClipEntity>)>,
    newly_added_entities: Query<Entity, (Added<Parent>, Without<ClipEntity>)>,
    mut commands: Commands,
) {
    for camera_entity in newly_added_cameras.iter() {
        let clip_entity_maybe = find_parent_clip(camera_entity, &parents, &clip_query);
        if let Some(clip_entity) = clip_entity_maybe {
            commands.entity(camera_entity).insert(ClipEntity(clip_entity));
        }
    }

    for new_entity in newly_added_entities.iter() {
        let clip_entity_maybe = find_parent_clip(new_entity, &parents, &clip_query);
        if let Some(clip_entity) = clip_entity_maybe {
            commands.entity(new_entity).insert(ClipEntity(clip_entity));
        }
    }
}

/// Filter out all visible entities that aren't children of the same clip as camera
pub fn clip_visibility_system(
    mut camera_query: Query<(&ClipEntity, &mut VisibleEntities), With<Camera>>,
    entity_query: Query<&ClipEntity>,
) {
    for (camera_clip_entity_component, mut visible_entities) in &mut camera_query {
        visible_entities.entities = visible_entities.entities.drain_filter(|entity| {
            if let Ok(child_clip_entity_component) = entity_query.get(*entity) {
                return camera_clip_entity_component.0 == child_clip_entity_component.0;
            }
            false
        }).collect();
    }
}