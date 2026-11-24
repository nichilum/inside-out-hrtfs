use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

use crate::ui::draw::{ShowcaseCamera, UiHandler, draw_gui, setup_3d_scene, update_speakers};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    // add code here
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_systems(Startup, setup_3d_scene)
            .add_systems(Update, update_speakers.run_if(on_message::<UpdateSpeakers>))
            .add_systems(EguiPrimaryContextPass, draw_gui)
            .add_systems(Update, (ShowcaseCamera::zoom, ShowcaseCamera::rotate))
            .init_resource::<UiHandler>();
    }
}

#[derive(Message, Debug)]
pub struct UpdateSpeakers(pub SpeakerOrder);

#[derive(Debug)]
pub enum SpeakerOrder {
    First,
    Second,
}
