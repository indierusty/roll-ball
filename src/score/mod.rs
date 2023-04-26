pub mod resources;
pub mod systems;

use bevy::app::Plugin;
use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Score>() // ::<> is called "Turbo-Fish Syntax"
            .init_resource::<HighScore>()
            .add_system(on_update_score)
            .add_system(update_high_score);
    }
}
