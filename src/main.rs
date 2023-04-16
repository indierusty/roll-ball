use bevy::{math::vec3, prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // player sprite size
pub const ENEMY_COUNT: usize = 5;
pub const ENEMY_SPEED: f32 = 250.0;
pub const ENEMY_SIZE: f32 = 64.0; // enemy sprite size
pub const STAR_COUNT: usize = 10;
pub const STAR_SIZE: f32 = 30.0; // star.png is 30x30 px
pub const STAR_SPAWN_TIME: f32 = 1.0; // sec

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>() // ::<> is called "Turbo-Fish Syntax"
        .init_resource::<StarSpawnTimer>()
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_startup_system(spawn_stars)
        .add_startup_system(spawn_camera)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemies_movement)
        .add_system(update_enemies_direction)
        .add_system(enemy_vs_player)
        .add_system(player_vs_stars)
        .add_system(on_update_score)
        .add_system(tick_spawn_star_timer)
        .add_system(spawn_star_over_time)
        .run()
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct Star {}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

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

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // we know that there is only one primary window hence get_single()
    let window = window_query.get_single().unwrap();

    for _ in 0..STAR_COUNT {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprite/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    // we know that there is only one primary window hence get_single()
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0),
        ..default()
    });
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

pub fn enemy_vs_player(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemies_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        for enemy_transform in enemies_query.iter() {
            let distance = ENEMY_SIZE / 2. + PLAYER_SIZE / 2.;
            let is_collided = enemy_transform
                .translation
                .distance_squared(player_transform.translation)
                <= distance * distance;

            if is_collided {
                println!("Game Over!");
                let sound_effect: Handle<AudioSource> =
                    asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
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

pub fn on_update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("You got star, score is {}", score.value)
    }
}

pub fn tick_spawn_star_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_star_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprite/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}
