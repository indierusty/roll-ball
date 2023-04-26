use super::resources::*;
use crate::events::*;
use bevy::prelude::*;

pub fn on_update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("You got star, score is {}", score.value)
    }
}

pub fn update_high_score(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_score: ResMut<HighScore>,
) {
    for event in game_over_event_reader.iter() {
        high_score.scores.push(("Player".to_string(), event.score))
    }
}
