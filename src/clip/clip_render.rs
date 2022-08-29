use bevy::prelude::*;
use bevy::render::camera::Projection;
use bevy::sprite::MaterialMesh2dBundle;
use crate::clip::Clip;

/// Renders a clip into a fullscreen quad
#[derive(Component)]
pub struct ClipRender {
    pub image: Handle<Image>,
}

pub fn setup_clip_renderer(
    mut commands: Commands,
    mut query: Query<&Clip, Added<Clip>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let clip = match query.iter().next() {
        Some(c) => c,
        None => return
    };

    let clip_render = ClipRender { image: clip.render_target.clone() };

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(clip.render_target.clone()),
        unlit: true,
        ..default()
    });

    let pbr_bundle = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 2.0 })),
        material: material_handle,
        ..default()
    };

    commands.spawn_bundle(pbr_bundle)
        .insert(clip_render);
}