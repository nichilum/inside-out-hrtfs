use bevy::prelude::*;

use crate::render::renderer::start_render_pool;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_render_pool);
    }
}
