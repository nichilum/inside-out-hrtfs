use bevy::prelude::*;

use crate::{render::plugin::RenderPlugin, ui::plugin::UiPlugin};

pub mod render;
pub mod ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin, RenderPlugin))
        .run();
}
