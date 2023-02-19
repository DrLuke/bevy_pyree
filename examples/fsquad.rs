//! Full screen shader effect

use bevy::prelude::*;
use bevy::render::render_resource::{AddressMode, Extent3d, SamplerDescriptor, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
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
use bevy::render::texture::ImageSampler;
use bevy::time::FixedTimestep;
use bevy_pyree::beat::{BeatEvent, OscBeatReceiverPlugin};


fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        watch_for_changes: true,
        ..default()
    }))
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        
        .add_plugin(MaterialPlugin::<MyExampleMaterial>::default())

        .add_startup_system(startup)
        .add_system(beat_system)
        // Send out a beat event once a second
        .add_event::<BeatEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(send_beat_event)
        )
        // Uncomment this if you want to receive OSC beat events instead
        //.add_plugin(BevyRoscPlugin::new("0.0.0.0:31337").unwrap())
        //.add_plugin(OscBeatReceiverPlugin::default())
    ;

    app.run();
}


#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "0a718bd5-e2f3-476e-b124-c4ade11c832c"]
pub struct MyExampleMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub previous_rt: Handle<Image>,
    #[uniform(2)]
    pub some_val: f32,
}

impl Material for MyExampleMaterial {
    fn fragment_shader() -> ShaderRef {
        "feedback_example.wgsl".into()
    }
}


fn startup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MyExampleMaterial>>,
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
        sampler_descriptor: ImageSampler::Descriptor(SamplerDescriptor {
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            ..Default::default()
        }),
        ..default()
    };
    image.resize(size);

    let rt_handle = images.add(image);

    let material_handle = materials.add(MyExampleMaterial {
        previous_rt: rt_handle.clone(),
        some_val: 0.0f32,
    });

    spawn_fs_quad::<MyExampleMaterial>(
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
        RenderLayers::layer(31),
    );
}

fn send_beat_event(
    mut ev: EventWriter<BeatEvent>
) {
    ev.send(BeatEvent {count: 0, bpm: None})
}

fn beat_system(
    mut ev: EventReader<BeatEvent>,
    material_query: Query<&FSQuad<MyExampleMaterial>>,
    mut materials: ResMut<Assets<MyExampleMaterial>>,
    mut toggle: Local<bool>
) {
    let fs_quad = material_query.single();
    let material = materials.get_mut(&fs_quad.material_handle).unwrap();

    for _ in ev.iter() {
        *toggle = !*toggle;
        if *toggle {
            material.some_val = 1.0;
        } else {
            material.some_val = 0.0;
        }
    }
}