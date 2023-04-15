use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .run()
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut cmds: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // we know that there is only one primary window hence get_single()
    let window = window_query.get_single().unwrap();

    cmds.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0),
            // asset_server automatically look in assets directory
            texture: asset_server.load("sprite/ball_blue_large.png"),
            ..default()
        },
        Player,
    ));
}

pub fn spawn_camera(mut cmds: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    // we know that there is only one primary window hence get_single()
    let window = window_query.get_single().unwrap();

    cmds.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0),
            ..default()
        },
        Player,
    ));
}
