use bevy::prelude::*;

use crate::{ui::plugin::UiPlugin, world::plugin::WorldPlugin};

pub mod components;
pub mod constants;
pub mod ui;
pub mod world;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin, WorldPlugin))
        .run();
}
