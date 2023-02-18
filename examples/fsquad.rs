//! Full screen shader effect

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::render::view::RenderLayers;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rosc::BevyRoscPlugin;
use bevy_pyree::render::{FSQuad, spawn_fs_quad, spawn_render_image_to_screen};
use bevy::reflect::TypeUuid;

use bevy::{
    render::{
        render_resource::{AsBindGroup, ShaderRef},
    },
};


fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        watch_for_changes: true,
        ..default()
    }))
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(BevyRoscPlugin::new("0.0.0.0:31337").unwrap())

        .add_plugin(MaterialPlugin::<FSQuadMaterial>::default())

        .add_startup_system(startup)
    ;

    app.run();
}


#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "0a718bd5-e2f3-476e-b124-c4ade11c832c"]
pub struct FSQuadMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub previous_rt: Handle<Image>,
    #[uniform(2)]
    pub some_val: f32,
}

impl Material for FSQuadMaterial {
    fn fragment_shader() -> ShaderRef {
        "shader.wgsl".into()
    }
}


fn startup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FSQuadMaterial>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
)
{
    let size = Extent3d { width: 1920, height: 1080, ..default() };
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };
    image.resize(size);

    let rt_handle = images.add(image);

    let material_handle = materials.add(FSQuadMaterial {
        previous_rt: rt_handle.clone(),
        some_val: 0.0f32,
    });

    spawn_fs_quad::<FSQuadMaterial>(
        &mut commands,
        &mut meshes,
        rt_handle.clone(),
        material_handle,
        1,
        0,
    );

    // Render to screen
    spawn_render_image_to_screen(
        &mut commands,
        &mut meshes,
        &mut std_materials,
        rt_handle,
        RenderLayers::layer(31)
    );
}