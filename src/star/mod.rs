pub mod components;
pub mod resources;
pub mod systems;

pub const STAR_SIZE: f32 = 30.0; // star.png is 30x30 px
const STAR_COUNT: usize = 10;

use bevy::app::Plugin;
use resources::*;
use systems::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<StarSpawnTimer>()
            .add_startup_system(spawn_stars)
            .add_system(tick_spawn_star_timer)
            .add_system(spawn_star_over_time);
    }
}
