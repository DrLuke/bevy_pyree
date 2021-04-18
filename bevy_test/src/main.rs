use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::{ActiveCameras, Camera, CameraProjection},
        pass::{
            LoadOp, Operations, PassDescriptor, RenderPassColorAttachmentDescriptor,
            RenderPassDepthStencilAttachmentDescriptor, TextureAttachment,
        },
        render_graph::{
            base::{node::MAIN_PASS, MainPass},
            CameraNode, Node, PassNode, RenderGraph, ResourceSlotInfo,
        },
        renderer::{RenderResourceId, RenderResourceType},
        texture::{
            Extent3d, SamplerDescriptor, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsage, SAMPLER_ASSET_INDEX, TEXTURE_ASSET_INDEX,
        },
    },
    window::WindowId,
};

mod render_to_texture_pipeline;

use render_to_texture_pipeline::{RenderToTextureGraphBuilder, RenderToTextureGraph};
use bevy::ecs::component::Component;
use bevy::asset::HandleId;

pub const RENDER_TEXTURE_HANDLE_UNTYPED: HandleUntyped = HandleUntyped::weak_from_u64(Texture::TYPE_UUID, 13378939762009864029);

pub const FIRST_PASS_CAMERA: &str = "first_pass_camera_test";


/// this component indicates what entities should rotate
struct Rotator;

struct Cube;

#[derive(Default)]
pub struct MyPass;


/// rotates the inner cube (first pass)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_x(1.5 * time.delta_seconds());
        transform.rotation *= Quat::from_rotation_z(1.3 * time.delta_seconds());
    }
}

/// rotates the outer cube (main pass)
fn cube_rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Cube>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_x(1.0 * time.delta_seconds());
        transform.rotation *= Quat::from_rotation_y(0.7 * time.delta_seconds());
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 4.0 }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        reflectance: 0.02,
        roughness: 1.0,
        unlit: false,
        ..Default::default()
    });

    commands
        .spawn_bundle(PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        })
        .insert(Rotator)
        .insert(MyPass)
        .remove::<MainPass>();
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..Default::default()
    });
    // camera

    let mut first_pass_camera = PerspectiveCameraBundle {
        camera: Camera {
            name: Some(FIRST_PASS_CAMERA.to_string()),
            window: WindowId::new(), // otherwise it will use main window size / aspect for calculation of projection matrix
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
            .looking_at(Vec3::default(), Vec3::Y),
        ..Default::default()
    };
    first_pass_camera.camera.window = WindowId::new();
    let camera_projection = &mut first_pass_camera.perspective_projection;
    camera_projection.update(1920.0, 1080.0);
    first_pass_camera.camera.projection_matrix = camera_projection.get_projection_matrix();
    first_pass_camera.camera.depth_calculation = camera_projection.depth_calculation();

    commands.spawn_bundle(first_pass_camera);

    let texture_handle = RENDER_TEXTURE_HANDLE_UNTYPED.typed::<Texture>();

    let cube_size = 4.0;
    let cube_handle = meshes.add(Mesh::from(shape::Box::new(cube_size, cube_size, cube_size)));

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        reflectance: 0.02,
        unlit: false,
        ..Default::default()
    });

    // add entities to the world
    commands
        .spawn_bundle(PbrBundle {
            mesh: cube_handle,
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.5),
                rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
                ..Default::default()
            },
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Cube);

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
            .looking_at(Vec3::default(), Vec3::Y),
        ..Default::default()
    });
}

fn main() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(cube_rotator_system.system())
        .add_system(rotator_system.system());
    {
        let world_cell = app.world_mut().cell();
        let mut render_graph = world_cell.get_resource_mut::<RenderGraph>().unwrap();
        let mut active_cameras = world_cell.get_resource_mut::<ActiveCameras>().unwrap();

        // Add each texture render pass
        render_graph.add_render_to_texture_graph::<MyPass>(&mut active_cameras, &RenderToTextureGraph{
            name: "test_pass",
            camera_name: FIRST_PASS_CAMERA,
            texture_handle: RENDER_TEXTURE_HANDLE_UNTYPED.typed::<Texture>(),
            width: 1920,
            height: 1080
        });
    }

    app.run();
}