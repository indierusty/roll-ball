use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use super::{components::*, *};

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // we know that there is only one primary window hence get_single()
    let window = window_query.get_single().unwrap();

    for _ in 0..ENEMY_COUNT {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprite/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: vec3(random(), random(), 0.0).normalize(),
            },
        ));
    }
}

pub fn enemies_movement(mut enemies_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemies_query.iter_mut() {
        transform.translation += enemy.direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemies_direction(
    mut enemies_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let enemy_radius = ENEMY_SIZE / 2.0;

    for (mut transform, mut enemy) in enemies_query.iter_mut() {
        let mut direction_changed = false;

        if transform.translation.x < enemy_radius
            || transform.translation.x > window.width() - enemy_radius
        {
            enemy.direction.x *= -1.;
            direction_changed = true;
        }

        if transform.translation.y < enemy_radius
            || transform.translation.y > window.height() - enemy_radius
        {
            enemy.direction.y *= -1.;
            direction_changed = true;
        }

        // playe sound and clamp position
        if direction_changed {
            // if enemy is far from window wall then enemy get stuck to wall, hence pull it inside window
            transform.translation = clamp_pos(transform.translation, enemy_radius + 1.0, window);

            // playe radius
            let sound_effect: Handle<AudioSource> = asset_server.load("audio/pluck_001.ogg");
            audio.play(sound_effect);
        }
    }
}

pub fn clamp_pos(pos: Vec3, radius: f32, window: &Window) -> Vec3 {
    let min = vec3(radius, radius, 0.);
    let max = vec3(window.width() - radius, window.height() - radius, 0.);
    pos.clamp(min, max)
}
