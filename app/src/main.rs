use bevy::prelude::*;

use crate::ui::plugin::UiPlugin;

pub mod ui;

fn main() {
    App::new().add_plugins((DefaultPlugins, UiPlugin)).run();
}
