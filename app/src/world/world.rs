use bevy::{
    camera::{CameraOutputMode, visibility::RenderLayers},
    prelude::*,
};
use bevy_egui::{EguiGlobalSettings, PrimaryEguiContext};
use wgpu_types::BlendState;

use crate::{components::speakers::UpdateSpeakers, world::camera::ShowcaseCamera};

pub fn setup_3d_scene(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Setup UpdateSpeakers Message
    commands.insert_resource(Messages::<UpdateSpeakers>::default());

    // Disable the automatic creation of a primary context to set it up manually for the camera we need.
    egui_global_settings.auto_create_primary_context = false;

    // Head import
    let head_handle = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("3d/head.gltf"),
    );

    commands.spawn((
        Mesh3d(head_handle),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 255, 255))),
        Transform::from_xyz(0., 0., 0.),
    ));

    // Point Lights
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            intensity: 10_000.,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -0.5),
    ));
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            intensity: 10_000.,
            ..default()
        },
        Transform::from_xyz(0.5, 0.0, 0.0),
    ));
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            intensity: 10_000.,
            ..default()
        },
        Transform::from_xyz(-0.5, 0.0, 0.5),
    ));
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            intensity: 5_000.,
            radius: 40.,
            ..default()
        },
        Transform::from_xyz(0.0, 0.5, 0.5),
    ));
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            intensity: 10_000.,
            radius: 40.,
            ..default()
        },
        Transform::from_xyz(0.0, 0.5, -0.5),
    ));

    // World camera.
    commands.spawn((Camera3d::default(), ShowcaseCamera::default()));
    // Egui camera.
    commands.spawn((
        // The `PrimaryEguiContext` component requires everything needed to render a primary context.
        PrimaryEguiContext,
        Camera2d,
        // Setting RenderLayers to none makes sure we won't render anything apart from the UI.
        RenderLayers::none(),
        Camera {
            order: 1,
            output_mode: CameraOutputMode::Write {
                blend_state: Some(BlendState::ALPHA_BLENDING),
                clear_color: ClearColorConfig::None,
            },
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
    ));
}
