use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::{ActiveCameras, Camera, CameraProjection, ScalingMode, DepthCalculation, OrthographicProjection},
        render_graph::{base::MainPass, RenderGraph},
        renderer::RenderResources,
    },
    window::WindowId,
};
use crate::pyree_config::PyreeConfig;
use crate::fs_quad::FsQuad;
use bevy::render::render_graph::base::node::MAIN_PASS;
use bevy::render::pipeline::{PipelineDescriptor, RenderPipeline};
use bevy::render::shader::ShaderStages;
use bevy::render::render_graph::{AssetRenderResourcesNode, RenderResourcesNode};
use crate::render_to_texture_pipeline::FIRST_PASS;


pub const RD_RT_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(Texture::TYPE_UUID, 13378939762009864035);
pub const RD_CAM_NAME: &str = "rd_cam";
pub const RD_GRAPH_NAME: &str = "rd_pass";

pub struct RdPass;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c620"]
pub struct RdMaterial {
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<RdMaterial>>,
    asset_server: ResMut<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut render_graph: ResMut<RenderGraph>,
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

    // Set up hot reloading shader
    asset_server.watch_for_changes().unwrap();
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: asset_server.load::<Shader, _>("shaders/rd.vert"),
        fragment: Some(asset_server.load::<Shader, _>("shaders/rd.frag")),
    }));
    render_graph.add_system_node(
        "rd_material",
        AssetRenderResourcesNode::<RdMaterial>::new(true),
    );

    render_graph
        .add_node_edge("rd_material", [RD_GRAPH_NAME, FIRST_PASS].concat())
        .unwrap();

    let material = materials.add(RdMaterial {});

    // FS quad
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(Mesh::from(FsQuad {})),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            pipeline_handle,
        )]),
        transform: Transform::from_rotation(Quat::from_rotation_x(0.0)),
        ..Default::default()
    }).insert(material)
        .insert(RdPass)
        .remove::<MainPass>();
}