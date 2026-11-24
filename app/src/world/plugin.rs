use bevy::prelude::*;

use crate::{
    components::speakers::UpdateSpeakers,
    ui::draw::UiHandler,
    world::{camera::ShowcaseCamera, speakers::update_speakers, world::setup_3d_scene},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_3d_scene)
            .add_systems(Update, update_speakers.run_if(on_message::<UpdateSpeakers>))
            .add_systems(Update, (ShowcaseCamera::zoom, ShowcaseCamera::rotate))
            .init_resource::<UiHandler>();
    }
}
