use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::{ActiveCameras, Camera, CameraProjection, ScalingMode, DepthCalculation, OrthographicProjection},
        render_graph::{base::MainPass, RenderGraph},
    },
    window::WindowId,
};
use crate::pyree_config::PyreeConfig;
use crate::fs_quad::FsQuad;
use bevy::render::render_graph::base::node::MAIN_PASS;


pub const RD_RT_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(Texture::TYPE_UUID, 13378939762009864035);
pub const RD_CAM_NAME: &str = "rd_cam";

pub struct RdPass;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn Orthographic camera
    let mut camera = OrthographicCameraBundle {
        camera: Camera {
            name: Some(RD_CAM_NAME.to_string()),
            window: WindowId::new(),
            ..Default::default()
        },
        orthographic_projection: OrthographicProjection {
            near: -1.0,
            far: 1.0,
            scaling_mode: ScalingMode::None,
            depth_calculation: DepthCalculation::ZDifference,
            ..Default::default()
        },
        visible_entities: Default::default(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        global_transform: Default::default(),
    };

    commands.spawn_bundle(camera);

    // FS quad
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(FsQuad {})),
        material: materials.add(Color::rgb(0.1, 0.1, 0.2).into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(0.0)),
        ..Default::default()
    }).insert(RdPass)
        .remove::<MainPass>();
}