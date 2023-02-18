use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::view::RenderLayers;

#[derive(Component)]
pub struct RenderToScreen {}

pub fn spawn_render_image_to_screen(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    image_handle: Handle<Image>,
    render_layer: RenderLayers,
) -> Entity {
    let render_mesh = MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 2.0 })),
        material: materials.add(StandardMaterial {
            unlit: true,
            base_color_texture: Some(image_handle),
            ..default()
        }),
        ..default()
    };
    commands
        .spawn((SpatialBundle::default()))
        .with_children(|child_builder| {
            child_builder.spawn((render_layer.clone(), render_mesh));
            child_builder.spawn((render_layer.clone(), Camera3dBundle {
                camera: Camera {
                    priority: isize::MAX,
                    ..default()
                },
                projection: Projection::Orthographic(OrthographicProjection {
                    scaling_mode: ScalingMode::None,
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Z),
                ..default()
            }));
        })
        .id()
}