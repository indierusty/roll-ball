pub mod components;
pub mod systems;

pub const ENEMY_COUNT: usize = 5;
pub const ENEMY_SPEED: f32 = 250.0;
pub const ENEMY_SIZE: f32 = 64.0; // enemy sprite size

use bevy::app::Plugin;
use systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_enemies)
            .add_system(enemies_movement)
            .add_system(update_enemies_direction);
    }
}
