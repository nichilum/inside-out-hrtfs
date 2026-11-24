use crate::components::speakers::{SpeakerOrder, UpdateSpeakers};
use bevy::{camera::Viewport, prelude::*, window::PrimaryWindow};
use bevy_egui::{
    EguiContext, EguiContexts,
    egui::{self, Color32, Frame, Margin},
};
use egui::{LayerId, Ui, UiBuilder};
use egui_scale::EguiScale;

#[derive(Resource, Default)]
pub struct UiHandler {
    pub cursor_over_ui: bool,
    pub enable_viewport_control: bool,
}

pub fn draw_gui(
    mut contexts: EguiContexts,
    mut camera: Single<&mut Camera, Without<EguiContext>>,
    window: Single<&mut Window, With<PrimaryWindow>>,
    mut ui_handler: ResMut<UiHandler>,
    mut update_speakers_writer: MessageWriter<UpdateSpeakers>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

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
                .fill(Color32::from_rgb(27, 27, 27)),
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
        .resizable(true)
        .show(&mut viewport_ui, |ui| {
            ui.style_mut().scale(1.5);
            // ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            if ui.button("Spawn Speakers").clicked() {
                update_speakers_writer.write(UpdateSpeakers(SpeakerOrder::First));
            }
            if ui.button("Spawn Speakers").clicked() {
                update_speakers_writer.write(UpdateSpeakers(SpeakerOrder::Second));
            }
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
