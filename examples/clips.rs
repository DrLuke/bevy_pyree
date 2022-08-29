//! Shows how to render to a texture. Useful for mirrors, UI, or exporting images.

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
use bevy_pyree::clip::Clip;
use bevy_pyree::clip::setup_clip_renderer;
use bevy::render::camera::{Projection, ScalingMode};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_clip_1)
        .add_startup_system(spawn_clip_2)
        .add_startup_system(setup)
        .add_system(setup_clip_renderer)
        .add_system(cube_rotator_system)
        .add_system(rotator_system)
        .run();
}

// Marks the first pass cube (rendered to a texture.)
#[derive(Component)]
struct Clip1Cube;

// Marks the main pass cube, to which the texture is applied.
#[derive(Component)]
struct Clip2Cube;

fn spawn_clip_1(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let clip = Clip::new(
        1,
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
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });

    commands
        .spawn_bundle(Camera3dBundle {
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

    commands.spawn().insert(clip);
}

fn spawn_clip_2(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let clip = Clip::new(
        2,
        images,
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
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });

    commands
        .spawn_bundle(Camera3dBundle {
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

    commands.spawn().insert(clip);
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera3dBundle {
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
