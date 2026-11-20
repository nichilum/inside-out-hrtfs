use bevy::{
    camera::{CameraOutputMode, Viewport, visibility::RenderLayers},
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::{
    EguiContext, EguiContexts, EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass,
    PrimaryEguiContext,
    egui::{self, Color32, Frame, Margin},
};
use egui::{LayerId, Ui, UiBuilder};
use wgpu_types::BlendState;

#[derive(Resource, Default)]
pub struct UiHandler {
    cursor_over_ui: bool,
    enable_viewport_control: bool,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(1., 1., 1.)))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, setup_system)
        .add_systems(EguiPrimaryContextPass, ui_example_system)
        .add_systems(Update, (ShowcaseCamera::zoom, ShowcaseCamera::rotate))
        .init_resource::<UiHandler>()
        .run();
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut camera: Single<&mut Camera, Without<EguiContext>>,
    window: Single<&mut Window, With<PrimaryWindow>>,
    mut ui_handler: ResMut<UiHandler>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    ctx.set_theme(egui::Theme::Light);

    let mut viewport_ui = Ui::new(
        ctx.clone(),
        "viewport".into(),
        UiBuilder::new()
            .layer_id(LayerId::background())
            .max_rect(ctx.viewport_rect()),
    );

    let mut top_menu = egui::Panel::top("top_panel")
        .frame(
            Frame::default()
                .inner_margin(Margin {
                    left: 5,
                    right: 5,
                    top: 2,
                    bottom: 2,
                })
                .fill(Color32::from_rgb(240, 240, 240)),
        )
        .show(&mut viewport_ui, |ui| {
            ui.horizontal(|ui| {
                ui.visuals_mut().button_frame = false;
                ui.menu_button("File", |ui| {
                    if ui.add(egui::Button::new("New")).clicked() {
                        ui.close();
                    }
                });
            });
        })
        .response
        .rect
        .height();

    let mut left = egui::Panel::left("left_panel")
        .default_size(400.0)
        .resizable(true)
        .show(&mut viewport_ui, |ui| {
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    // Scale from logical units to physical units.
    left *= window.scale_factor();
    top_menu *= window.scale_factor();

    let pos = UVec2::new(left as u32, top_menu as u32);
    let size = UVec2::new(window.physical_width(), window.physical_height()) - pos;

    camera.viewport = Some(Viewport {
        physical_position: pos,
        physical_size: size,
        ..default()
    });

    // TODO: this is horrible
    if let Some(pointer_pos) = viewport_ui.pointer_latest_pos() {
        ui_handler.cursor_over_ui = pointer_pos.x < left + 10. || pointer_pos.y < top_menu + 5.;
    };
    Ok(())
}

fn setup_system(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Disable the automatic creation of a primary context to set it up manually for the camera we need.
    egui_global_settings.auto_create_primary_context = false;

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

    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            intensity: 100_000.,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 4.0),
    ));
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -4.0),
    ));
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 0.0, 0.0),
    ));
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-4.0, 0.0, 4.0),
    ));
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 4.0, 4.0),
    ));
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, -4.0, 4.0),
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

#[derive(Component, Reflect, Clone, PartialEq, Debug)]
pub struct ShowcaseCamera {
    pub orbit: Vec3,
    pub distance: f32,
    pub mouse_sensitivity: f32,
    pub zoom_scale: f32,
}
impl ShowcaseCamera {
    pub fn rotate(
        mut mouse_motion_events: MessageReader<MouseMotion>,
        mouse_input: Res<ButtonInput<MouseButton>>,
        mut query: Query<(&ShowcaseCamera, &mut Transform)>,
        mut ui_handler: ResMut<UiHandler>,
    ) {
        let mut delta = bevy::math::Vec2::ZERO;

        if mouse_input.pressed(MouseButton::Left) && ui_handler.enable_viewport_control {
            delta = mouse_motion_events.read().map(|e| e.delta).sum();
        }
        if mouse_input.just_pressed(MouseButton::Left) {
            ui_handler.enable_viewport_control = !ui_handler.cursor_over_ui;
            delta = bevy::math::Vec2::ZERO;
        }
        for (camera, mut transform) in &mut query {
            // ROTATION
            let (mut rx, mut ry, rz) = transform.rotation.to_euler(EulerRot::YXZ);
            rx += (-delta.x * camera.mouse_sensitivity).to_radians();
            ry += (-delta.y * camera.mouse_sensitivity).to_radians();
            ry = ry.clamp(-90_f32.to_radians(), 90_f32.to_radians());
            transform.rotation = Quat::from_euler(EulerRot::YXZ, rx, ry, rz);

            // ORBIT TRANSFORM
            let tx = camera.distance * rx.sin();
            let ty = camera.distance * rx.cos();
            let tz = camera.distance * ry.sin();

            let diff = camera.distance * ry.cos();
            let plane_ratio_decrease = (camera.distance - diff) / camera.distance;

            transform.translation = camera.orbit;
            transform.translation.x += tx * (1.0 - plane_ratio_decrease);
            transform.translation.z += ty * (1.0 - plane_ratio_decrease);
            transform.translation.y += -tz;
        }
    }
    pub fn zoom(
        mut mouse_wheel_events: MessageReader<MouseWheel>,
        mut query: Query<&mut ShowcaseCamera>,
    ) {
        let delta: f32 = mouse_wheel_events.read().map(|e| e.y).sum();
        for mut camera in &mut query {
            if camera.zoom_scale != 0.0 {
                camera.distance -= delta * camera.distance / camera.zoom_scale;
            }
        }
    }
}
impl Default for ShowcaseCamera {
    fn default() -> Self {
        Self {
            orbit: Vec3::new(0., 0.2, 0.),
            distance: 2.0,
            mouse_sensitivity: 0.1,
            zoom_scale: 25.0,
        }
    }
}
