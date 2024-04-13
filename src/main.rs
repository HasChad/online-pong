use bevy::{prelude::*, window::WindowMode};
use bevy_kira_audio::prelude::*;

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;
const PLAYER_WIDTH: f32 = 30.0;
const PLAYER_HEIGHT: f32 = 200.0;
const BALL_DIAMETER: f32 = 40.0;

// ! Fill those structs with collision, position and point
#[derive(Component)]
struct PlayerLeft;

#[derive(Component)]
struct PlayerRight;

#[derive(Component)]
struct Ball {
    velocity_x: f32,
    velocity_y: f32,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pong Online".into(),
                        mode: WindowMode::Windowed,
                        resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                        resizable: false,
                        position: WindowPosition::Centered(MonitorSelection::Primary),

                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(AudioPlugin)
        .add_systems(Startup, game_setup)
        .add_systems(
            Update,
            (ball_movement, playerleft_movement, playerright_movement),
        )
        .run();
}

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    commands.spawn(Camera2dBundle::default());
    audio
        .play(asset_server.load("sounds/game-music.ogg"))
        .looped();

    //Background
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/background.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT)),
                ..default()
            },
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 2.0,
        },
    ));

    //Middle Line
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/border.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(20.0, SCREEN_HEIGHT)),
                ..default()
            },
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: false,
            tile_y: true,
            stretch_value: 2.0,
        },
    ));

    //Player Left
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/playerLeft.png"),
            transform: Transform::from_xyz(0.0 - SCREEN_WIDTH / 2.0 + 25.0, 0.0, 1.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                ..default()
            },
            ..default()
        },
        PlayerLeft,
    ));

    //Player Right
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/playerRight.png"),
            transform: Transform::from_xyz(SCREEN_WIDTH / 2.0 - 25.0, 0.0, 1.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                ..default()
            },
            ..default()
        },
        PlayerRight,
    ));

    //Ball
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/ball.png"),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(BALL_DIAMETER, BALL_DIAMETER)),
                ..default()
            },
            ..default()
        },
        Ball {
            velocity_x: 300.0,
            velocity_y: 0.0,
        },
    ));
}

fn ball_movement(mut ball_query: Query<(&mut Ball, &mut Transform)>, time: Res<Time>) {
    for (mut ball, mut ball_tranform) in ball_query.iter_mut() {
        ball_tranform.translation.x += ball.velocity_x * time.delta_seconds();

        if ball_tranform.translation.x > (SCREEN_WIDTH / 2.0) - 60.0 {
            ball.velocity_x = -ball.velocity_x;
        }
        if ball_tranform.translation.x < 0.0 - (SCREEN_WIDTH / 2.0) + 60.0 {
            ball.velocity_x = -ball.velocity_x;
        }
    }
}

fn playerleft_movement(
    mut playerleft_query: Query<&mut Transform, With<PlayerLeft>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut playerleft_transform = playerleft_query.single_mut();

    if input.pressed(KeyCode::KeyW)
        && playerleft_transform.translation.y + PLAYER_HEIGHT / 2.0 < SCREEN_HEIGHT / 2.0
    {
        playerleft_transform.translation.y += 0.5
    }
    if input.pressed(KeyCode::KeyS)
        && playerleft_transform.translation.y - PLAYER_HEIGHT / 2.0 > -SCREEN_HEIGHT / 2.0
    {
        playerleft_transform.translation.y -= 0.5
    }
}

fn playerright_movement(
    mut playerright_query: Query<&mut Transform, With<PlayerRight>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut playerright_transform = playerright_query.single_mut();

    if input.pressed(KeyCode::ArrowUp)
        && playerright_transform.translation.y + PLAYER_HEIGHT / 2.0 < SCREEN_HEIGHT / 2.0
    {
        playerright_transform.translation.y += 0.5
    }
    if input.pressed(KeyCode::ArrowDown)
        && playerright_transform.translation.y - PLAYER_HEIGHT / 2.0 > -SCREEN_HEIGHT / 2.0
    {
        playerright_transform.translation.y -= 0.5
    }
}
