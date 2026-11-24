use bevy::prelude::*;

use crate::{render::plugin::RenderPlugin, ui::plugin::UiPlugin, world::plugin::WorldPlugin};
pub mod components;
pub mod constants;
pub mod render;
pub mod ui;
pub mod world;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin, WorldPlugin, RenderPlugin))
        .run();
}
