use super::{components::Player, *};
use crate::enemy::components::Enemy;
use crate::enemy::ENEMY_SIZE;
use crate::events::GameOver;
use crate::score::resources::*;
use crate::star::components::Star;
use crate::star::STAR_SIZE;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // we know that there is only one primary window hence get_single()
    let window = window_query.get_single().unwrap();

    commands.spawn((
        // Bundle is pack of components. where trasform is one of component.
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0),
            // asset_server automatically look in assets directory
            texture: asset_server.load("sprite/ball_blue_large.png"),
            ..default()
        },
        Player,
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            direction.y = 1.;
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            direction.y = -1.;
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            direction.x = -1.;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            direction.x = 1.;
        }

        // in case direction is zero we should not normalize hence
        direction = direction.normalize_or_zero();

        transform.translation += direction * (PLAYER_SPEED * time.delta_seconds());
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let player_radius = PLAYER_SIZE / 2.;
        let min = vec3(player_radius, player_radius, 0.);
        let max = vec3(
            window.width() - player_radius,
            window.height() - player_radius,
            0.,
        );

        transform.translation = transform.translation.clamp(min, max);
    }
}
pub fn enemy_vs_player(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemies_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut game_over_event_writer: EventWriter<GameOver>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        for enemy_transform in enemies_query.iter() {
            let distance = ENEMY_SIZE / 2. + PLAYER_SIZE / 2.;
            let is_collided = enemy_transform
                .translation
                .distance_squared(player_transform.translation)
                <= distance * distance;

            if is_collided {
                let sound_effect: Handle<AudioSource> =
                    asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn player_vs_stars(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    stars_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in stars_query.iter() {
            let distance = STAR_SIZE / 2. + PLAYER_SIZE / 2.;

            let is_collided = star_transform
                .translation
                .distance_squared(player_transform.translation)
                <= distance * distance;

            if is_collided {
                let sound_effect: Handle<AudioSource> =
                    asset_server.load("audio/laserLarge_000.ogg");
                audio.play(sound_effect);
                commands.entity(star_entity).despawn();

                score.value += 1;
            }
        }
    }
}
