//! Shows how to render to a texture. Useful for mirrors, UI, or exporting images.

use std::ops::Deref;
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};
use bevy_pyree::clip::{Clip, ClipLayer, ClipLayerBundle, ClipLayerLastRenderTarget, ClipLayerMaterial, ClipRender, Deck2, Deck2Material, DeckRenderer, extract_deck2, ExtractedCrossfade, prepare_deck2, PyreeClipPlugin, setup_deck2, spawn_clip_layer_bundle};
use bevy_pyree::clip::setup_clip_renderer;
use bevy::render::camera::{Projection, ScalingMode};
use bevy::render::{RenderApp, RenderStage};
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_egui::egui::TextureId;
use bevy_inspector_egui::WorldInspectorPlugin;
use crate::egui::emath;


fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(MaterialPlugin::<Deck2Material>::default())
        .add_plugin(ExtractResourcePlugin::<ExtractedCrossfade>::default())

        .add_plugin(WorldInspectorPlugin::default())

        .add_plugin(PyreeClipPlugin)

        //.add_startup_system(spawn_clip_1)
        //.add_startup_system(spawn_clip_2)
        //.add_startup_system(spawn_clip_3)
        .add_startup_system(setup)
        .add_startup_system(image_clip)
        //.add_system(setup_clip_renderer)
        .add_system(cube_rotator_system)
        .add_system(rotator_system)
        .add_system(cube_rotator_system_also)
        //.add_system(clip_selector_gui)
        .add_system(deck_system)
        .add_system(deck_crossfader)
        .add_system(clip_layer_ui)
        //.add_system(deck_gui)

        .add_startup_system(setup_deck2);

    app.sub_app_mut(RenderApp)
        .add_system_to_stage(RenderStage::Extract, extract_deck2)
        .add_system_to_stage(RenderStage::Prepare, prepare_deck2);

    app.run();
}

pub fn image_clip(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ClipLayerMaterial>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut layer0 = ClipLayerBundle::new(
        0,
        &mut materials,
        &mut meshes,
        &mut images,
    );

    let clip = Clip::from_image("Clip1".into(), server.load("Clip1.png"));
    let rt = clip.render_target.clone();
    layer0.clip_layer.add_clip(0, commands.spawn(clip).id(), rt);
    let clip = Clip::from_image("Clip2".into(), server.load("Clip2.png"));
    let rt = clip.render_target.clone();
    layer0.clip_layer.add_clip(1, commands.spawn(clip).id(), rt);
    let clip = Clip::from_image("Clip3".into(), server.load("Clip3.png"));
    let rt = clip.render_target.clone();
    layer0.clip_layer.add_clip(2, commands.spawn(clip).id(), rt);

    let (dyn_clip, rt) = spawn_clip_1(&mut commands, &mut meshes, &mut std_materials, &mut images);
    layer0.clip_layer.add_clip(3, dyn_clip, rt);

    let mut layer1 = ClipLayerBundle::new(
        1,
        &mut materials,
        &mut meshes,
        &mut images,
    );
    let clip = Clip::from_image("Clip1".into(), server.load("Clip4.png"));
    let rt = clip.render_target.clone();
    layer1.clip_layer.add_clip(0, commands.spawn(clip).id(), rt);
    let clip = Clip::from_image("Clip2".into(), server.load("Clip5.png"));
    let rt = clip.render_target.clone();
    layer1.clip_layer.add_clip(1, commands.spawn(clip).id(), rt);
    let clip = Clip::from_image("Clip3".into(), server.load("Clip6.png"));
    let rt = clip.render_target.clone();
    layer1.clip_layer.add_clip(2, commands.spawn(clip).id(), rt);

    let (dyn_clip, rt) = spawn_clip_2(&mut commands, &mut meshes, &mut std_materials, &mut images);
    layer1.clip_layer.add_clip(3, dyn_clip, rt);

    let mut layer2 = ClipLayerBundle::new(
        2,
        &mut materials,
        &mut meshes,
        &mut images,
    );
    let clip = Clip::from_image("Clip1".into(), server.load("Clip7.png"));
    let rt = clip.render_target.clone();
    layer2.clip_layer.add_clip(0, commands.spawn(clip).id(), rt);
    let clip = Clip::from_image("Clip2".into(), server.load("Clip8.png"));
    let rt = clip.render_target.clone();
    layer2.clip_layer.add_clip(1, commands.spawn(clip).id(), rt);
    let clip = Clip::from_image("Clip3".into(), server.load("Clip9.png"));
    let rt = clip.render_target.clone();
    layer2.clip_layer.add_clip(2, commands.spawn(clip).id(), rt);

    let (dyn_clip, rt) = spawn_clip_3(&mut commands, &mut meshes, &mut std_materials, &mut images);
    layer2.clip_layer.add_clip(3, dyn_clip, rt);

    spawn_clip_layer_bundle(&mut commands, layer0, 20);
    spawn_clip_layer_bundle(&mut commands, layer1, 21);
    spawn_clip_layer_bundle(&mut commands, layer2, 22);
}

pub fn deck_system(
    deck: Res<Deck2>,
    query: Query<(Entity, &DeckRenderer, &Handle<Deck2Material>)>,
    clip_query: Query<&Clip>,
    mut commands: Commands,
    mut materials: ResMut<Assets<Deck2Material>>,
    output_rt: ResMut<ClipLayerLastRenderTarget>,
) {
    if deck.is_changed() {
        for (entity, deck_renderer, handle) in query.iter() {
            let mut tex1 = None;
            let mut tex2 = None;
            if deck.slot_a.is_some() {
                tex1 = Some(clip_query.get(deck.slot_a.unwrap()).unwrap().render_target.clone());
            }
            if deck.slot_b.is_some() {
                tex2 = Some(clip_query.get(deck.slot_b.unwrap()).unwrap().render_target.clone());
            }


            commands.entity(entity)
                .remove::<Handle<Deck2Material>>()
                .insert(
                    materials.add(Deck2Material {
                        fade_ab: deck.crossfade.clone(),
                        image_a: output_rt.render_target.clone(),
                        image_b: tex2,
                    })
                );
        }
    }
}

// Marks the first pass cube (rendered to a texture.)
#[derive(Component)]
struct Clip1Cube;

// Marks the main pass cube, to which the texture is applied.
#[derive(Component)]
struct Clip2Cube;

#[derive(Component)]
struct Clip3Cube;

fn spawn_clip_1(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut images: &mut ResMut<Assets<Image>>,
) -> (Entity, Handle<Image>) {
    let clip = Clip::new(
        "Clip 1".into(),
        images,
        Extent3d {
            width: 1920,
            height: 1080,
            ..default()
        },
    );

    // Render layer
    let rl = RenderLayers::layer(1);

    // Just some geometry to display
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 4.0 }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.15, 0.1),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    // The cube that will be rendered to the texture.
    commands
        .spawn_bundle(PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        })
        .insert(Clip1Cube)
        .insert(rl);

    // Light
    // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });

    commands
        .spawn(Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::WHITE),
                ..default()
            },
            camera: Camera {
                priority: 0,
                target: RenderTarget::Image(clip.render_target.clone()),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        .insert(rl);

    let rt = clip.render_target.clone();
    (commands.spawn(clip).id(), rt)
}

fn spawn_clip_2(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut images: &mut ResMut<Assets<Image>>,
) -> (Entity, Handle<Image>) {
    let clip = Clip::new(
        "Another Clip".into(),
        &mut images,
        Extent3d {
            width: 1920,
            height: 1080,
            ..default()
        },
    );

    // Render layer
    let rl = RenderLayers::layer(2);

    // Just some geometry to display
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 4.0 }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.1, 0.95, 0.05),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    // The cube that will be rendered to the texture.
    commands
        .spawn_bundle(PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        })
        .insert(Clip2Cube)
        .insert(rl);

    // Light
    // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });

    commands
        .spawn(Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::WHITE),
                ..default()
            },
            camera: Camera {
                priority: 0,
                target: RenderTarget::Image(clip.render_target.clone()),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        .insert(rl);

    let rt = clip.render_target.clone();
    (commands.spawn(clip).id(), rt)
}

fn spawn_clip_3(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut images: &mut ResMut<Assets<Image>>,
) -> (Entity, Handle<Image>) {
    let clip = Clip::new(
        "Torus".into(),
        &mut images,
        Extent3d {
            width: 1920,
            height: 1080,
            ..default()
        },
    );

    // Render layer
    let rl = RenderLayers::layer(3);

    // Just some geometry to display
    let cube_handle = meshes.add(Mesh::from(shape::Torus {
        radius: 2.0,
        ring_radius: 0.4,
        subdivisions_segments: 10,
        subdivisions_sides: 10,
    }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 0.1, 0.95),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    // The cube that will be rendered to the texture.
    commands
        .spawn_bundle(PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        })
        .insert(Clip3Cube)
        .insert(rl);

    // Light
    // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });

    commands
        .spawn(Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::WHITE),
                ..default()
            },
            camera: Camera {
                priority: 0,
                target: RenderTarget::Image(clip.render_target.clone()),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        .insert(rl);

    let rt = clip.render_target.clone();
    (commands.spawn(clip).id(), rt)
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera3dBundle {
        camera: Camera {
            priority: 1,
            ..default()
        },
        projection: Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::None,
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)).looking_at(Vec3::default(), Vec3::Z),
        ..default()
    });

    commands.insert_resource(Deck2::default());
}

/// Rotates the inner cube (first pass)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Clip1Cube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.5 * time.delta_seconds());
        transform.rotate_z(1.3 * time.delta_seconds());
    }
}

/// Rotates the outer cube (main pass)
fn cube_rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Clip2Cube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.0 * time.delta_seconds());
        transform.rotate_y(0.7 * time.delta_seconds());
    }
}

/// Rotates the outer cube (main pass)
fn cube_rotator_system_also(time: Res<Time>, mut query: Query<&mut Transform, With<Clip3Cube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.01293192 * time.delta_seconds());
        transform.rotate_y(0.5341 * time.delta_seconds());
        transform.rotate_y(0.1 * time.delta_seconds());
    }
}

fn clip_selector_gui(
    mut egui_context: ResMut<EguiContext>,
    clip_query: Query<(Entity, &Clip)>,
    mut deck: ResMut<Deck2>,
) {
    egui::Window::new("Deck Clip selector").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            for (entity, clip) in clip_query.iter() {
                let mut button = egui::Button::new(clip.get_display_name());

                if deck.slot_a.is_some() && deck.slot_a.unwrap() == entity {
                    button = button.frame(false);
                }
                if ui.add(button).clicked() {
                    deck.slot_a = Some(entity);
                }
            }
        });
        ui.horizontal(|ui| {
            for (entity, clip) in clip_query.iter() {
                let mut button = egui::Button::new(clip.get_display_name());

                if deck.slot_b.is_some() && deck.slot_b.unwrap() == entity {
                    button = button.frame(false);
                }
                if ui.add(button).clicked() {
                    deck.slot_b = Some(entity);
                }
            }
        });
    });
}

/*fn deck_gui(
    deck: ResMut<Deck2>,
    mut egui_context: ResMut<EguiContext>,
    mut clip_render_query: Query<(Entity, &mut ClipRender)>,
    clip_query: Query<&Clip>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (entity, mut clip_render) = match clip_render_query.iter_mut().next() {
        Some(c) => c,
        None => {
            egui::Window::new("Uh oh").show(egui_context.ctx_mut(), |ui| {
                ui.label("No clip Renderer found");
            });
            return;
        }
    };

    let mut set_clip = |clip: &Clip, commands: &mut Commands, materials: &mut ResMut<Assets<StandardMaterial>>| {
        clip_render.image = clip.render_target.clone();
        commands.entity(entity).remove::<Handle<StandardMaterial>>();
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(clip.render_target.clone()),
            unlit: true,
            ..default()
        });
        commands.entity(entity).insert(material_handle);
    };

    egui::Window::new("Deck").show(egui_context.ctx_mut(), |ui| {
        egui::Grid::new("some_unique_id").show(ui, |ui| {
            if ui.button("Deck A").clicked() {
                if deck.slots[0].is_some() {
                    set_clip(clip_query.get(deck.slots[0].unwrap()).unwrap(), &mut commands, &mut materials);
                }
            }
            ui.label("Fader hier");
            if ui.button("Deck B").clicked() {
                if deck.slots[1].is_some() {
                    set_clip(clip_query.get(deck.slots[1].unwrap()).unwrap(), &mut commands, &mut materials);
                }
            }
            ui.end_row();
        });
    });
}*/

pub fn deck_crossfader(
    mut deck: ResMut<Deck2>,
    mut egui_context: ResMut<EguiContext>,
) {
    egui::Window::new("Crossfade").show(egui_context.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut deck.crossfade, 0.0..=1.0).text("value"));
    });
}

pub fn clips_images(
    query: Query<(Entity, &Clip)>,
    mut egui_context: ResMut<EguiContext>,
    mut deck: ResMut<Deck2>,
) {
    let mut image_ids = vec![];
    let mut selected_index = None;
    for (i, (entity, clip)) in query.iter().enumerate() {
        let rt = clip.render_target.clone();
        match egui_context.image_id(&rt) {
            None => image_ids.push(egui_context.add_image(rt)),
            Some(id) => image_ids.push(id)
        }

        if let Some(selected) = deck.slot_a {
            if selected == entity {
                selected_index = Some(i);
            }
        }
    }

    egui::Window::new("Clips").show(egui_context.ctx_mut(), |ui| {
        for (i, id) in image_ids.iter().enumerate() {
            if ui.add(egui::widgets::ImageButton::new(
                *id,
                emath::Vec2::new(100., 100.),
            )
                .selected(selected_index.unwrap_or(100) == i)
            ).clicked() {
                for (j, (entity, _)) in query.iter().enumerate() {
                    if j == i {
                        deck.slot_a = Some(entity);
                    }
                }
            }
        }
    });
}

pub fn clip_layer_ui(
    mut query: Query<&mut ClipLayer>,
    mut egui_context: ResMut<EguiContext>,
    mut deck: ResMut<Deck2>,
) {
    for mut cl in query.iter_mut() {
        let mut image_ids = vec![];
        let mut selected_index: u8 = cl.get_active_clip();

        for rt in cl.get_render_targets() {
            if let Some(handle) = rt {
                match egui_context.image_id(&handle) {
                    None => image_ids.push(Some(egui_context.add_image(handle))),
                    Some(id) => image_ids.push(Some(id))
                }
            } else {
                image_ids.push(None);
            }
        }

        'l: loop {
            match image_ids.pop() {
                None => break 'l,
                Some(maybe_texture_id) => match maybe_texture_id {
                    None => continue,
                    Some(texture_id) => {
                        image_ids.push(Some(texture_id));
                        break 'l;
                    }
                }
            }
        }

        egui::Window::new(format!("Layer {}", cl.layer)).show(egui_context.ctx_mut(), |ui| {
            ui.add(egui::Slider::new(&mut cl.blend, 0.0..=1.0).text("value"));
            for (i, id_maybe) in image_ids.iter().enumerate() {
                if let Some(id) = id_maybe {
                    if ui.add(egui::widgets::ImageButton::new(
                        *id,
                        emath::Vec2::new(100., 100.),
                    )
                        .selected(selected_index as usize == i)
                    ).clicked() {
                        for (j, clip) in cl.get_clips().iter().enumerate() {
                            if j == i {
                                match cl.layer {
                                    0 => deck.slot_a = *clip,
                                    1 => deck.slot_b = *clip,
                                    _ => {}
                                }
                                cl.set_active_clip(j as u8);
                            }
                        }
                    }
                }
            }
        });
    }
}