use crate::get_window_dimensions;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use std::io::Cursor;
use winit::window::Icon;

use crate::{Ball, Border, Paddle, Score, Side, Velocity, PADDLE_MARGIN};

// Sets the icon on windows and X11
pub fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!("../assets/icon.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}

// Spawns the camera that draws UI
pub fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

pub fn setup_game(mut commands: Commands, windows: Query<&Window>) {
    let (half_window_width, half_window_height) =
        get_window_dimensions(windows.iter().next().unwrap());
    let paddle_x = half_window_width - PADDLE_MARGIN;

    // Spawn the left paddle
    const LEFT_PADDLE: Paddle = Paddle { side: Side::Left };
    const RIGHT_PADDLE: Paddle = Paddle { side: Side::Right };
    let paddle_sprite: SpriteBundle = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(10.0, 100.0)),
            ..Default::default()
        },
        ..Default::default()
    };

    commands
        .spawn(LEFT_PADDLE)
        .insert(Transform::from_xyz(-paddle_x, 0.0, 0.1))
        .insert(paddle_sprite.clone());

    // Spawn the right paddle
    commands
        .spawn(RIGHT_PADDLE)
        .insert(Transform::from_xyz(paddle_x, 0.0, 0.1))
        .insert(paddle_sprite.clone());

    // Spawn the ball
    let quarter_window_width = half_window_width / 2.0;
    let quarter_window_height = half_window_height / 2.0;
    commands
        .spawn(Ball)
        .insert(Transform::from_xyz(0.0, 0.0, 0.1))
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Velocity {
            x: quarter_window_width,
            y: quarter_window_height,
        });

    const LEFT_SCORE: Score = Score {
        value: 0,
        side: Side::Left,
    };
    // Spawn the left score
    commands.spawn(LEFT_SCORE).insert(
        TextBundle::from_sections([TextSection {
            style: TextStyle {
                font_size: 100.0,
                color: Color::rgb(1.0, 1.0, 1.0),
                ..Default::default()
            },
            value: "0".to_string(),
        }])
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Percent(40.0),
            top: Val::Px(20.0),
            ..Default::default()
        }),
    );

    const RIGHT_SCORE: Score = Score {
        value: 0,
        side: Side::Right,
    };
    // Spawn the right score
    commands.spawn(RIGHT_SCORE).insert(
        TextBundle::from_sections([TextSection {
            style: TextStyle {
                font_size: 100.0,
                color: Color::rgb(1.0, 1.0, 1.0),
                ..Default::default()
            },
            value: "0".to_string(),
        }])
        .with_style(Style {
            position_type: PositionType::Absolute,
            right: Val::Percent(40.0),
            top: Val::Px(20.0),
            ..Default::default()
        }),
    );

    //draw border
    let border_thickness = 5.0;
    let border_color = Color::rgb(1.0, 1.0, 1.0);

    // Draw the middle dashed line
    let dash_length = 5.0;
    let dash_spacing = 5.0;
    let num_dashes = (half_window_height * 3.0 / (dash_length + dash_spacing)).ceil() as u32;
    for i in 0..num_dashes {
        let y = half_window_height * 1.5 - (i as f32 * (dash_length + dash_spacing));
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: border_color,
                    custom_size: Some(Vec2::new(border_thickness, dash_length)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, y, 0.0),
                ..Default::default()
            })
            .insert(Border);
    }
}
