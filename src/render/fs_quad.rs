use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::{RenderTarget, ScalingMode};
use bevy::render::view::{ColorGrading, RenderLayers};

/// Contains the render target for a fullscreen quad
#[derive(Component)]
pub struct FSQuad<T: Material> {
    pub render_target: Handle<Image>,
    pub material_handle: Handle<T>,
}

/// Spawns a Fullscreen quad with a material applied to it, and has it rendered to `render_target`
pub fn spawn_fs_quad<T: Material>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    render_target: Handle<Image>,
    material_handle: Handle<T>,
    render_layer: u8,
    render_priority: isize,
) -> Entity {
    let fs_quad = FSQuad::<T> {
        render_target,
        material_handle: material_handle.clone(),
    };
    let rt_handle = fs_quad.render_target.clone();

    commands
        .spawn((fs_quad, SpatialBundle::default()))
        .with_children(|child_builder| {
            child_builder
                .spawn(Camera3dBundle {
                    camera: Camera {
                        order: render_priority,
                        target: RenderTarget::Image(rt_handle.clone()),
                        ..default()
                    },
                    projection: Projection::Orthographic(OrthographicProjection {
                        scaling_mode: ScalingMode::Fixed {width:1., height:1.},
                        ..default()
                    }),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                        .looking_at(Vec3::default(), Vec3::Y),
                    tonemapping: Tonemapping::None,
                    ..default()
                })
                .insert(RenderLayers::layer(render_layer));
            child_builder
                .spawn(MaterialMeshBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())),
                    material: material_handle,
                    ..default()
                })
                .insert(RenderLayers::layer(render_layer));
        })
        .id()
}
