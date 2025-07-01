use crate::get_window_dimensions;
use bevy::{prelude::*, window::PrimaryWindow, ui::{Node, Val}};
use bevy::winit::WinitWindows;
use std::io::Cursor;
use winit::window::Icon;

use crate::{Ball, Border, Paddle, Score, Side, Velocity, PADDLE_MARGIN};

// Sets the icon on windows and X11
pub fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single().expect("Primary window not found");
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!("../assets/icon.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        // Construct winit::window::Icon directly
        if let Ok(icon) = Icon::from_rgba(rgba, width, height) {
            primary.set_window_icon(Some(icon));
        }
    };
}

// Spawns the camera that draws UI
pub fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2d);
}

pub fn setup_game(mut commands: Commands, windows: Query<&Window>) {
    let (half_window_width, half_window_height) =
        get_window_dimensions(windows.iter().next().unwrap());
    let paddle_x = half_window_width - PADDLE_MARGIN;

    // Spawn the left paddle
    const LEFT_PADDLE: Paddle = Paddle { side: Side::Left };
    const RIGHT_PADDLE: Paddle = Paddle { side: Side::Right };

    commands.spawn((
        LEFT_PADDLE,
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(10.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(-paddle_x, 0.0, 0.1),
    ));

    // Spawn the right paddle
    commands.spawn((
        RIGHT_PADDLE,
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(10.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(paddle_x, 0.0, 0.1),
    ));

    // Spawn the ball
    let quarter_window_width = half_window_width / 2.0;
    let quarter_window_height = half_window_height / 2.0;
    commands.spawn((
        Ball,
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(10.0, 10.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.1),
        Velocity {
            x: quarter_window_width,
            y: quarter_window_height,
        },
    ));

    const LEFT_SCORE: Score = Score {
        value: 0,
        side: Side::Left,
    };
    // Spawn the left score
    commands.spawn((
        LEFT_SCORE,
        Text::new(
            "0".to_string(),
        ),
        TextFont {
            font_size: 100.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(40.0),
            top: Val::Px(20.0),
            ..default()
        },
    ));

    const RIGHT_SCORE: Score = Score {
        value: 0,
        side: Side::Right,
    };
    // Spawn the right score
    commands.spawn((
        RIGHT_SCORE,
        Text::new(
            "0".to_string(),
        ),
        TextFont {
            font_size: 100.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            right: Val::Percent(40.0),
            top: Val::Px(20.0),
            ..default()
        },
    ));

    //draw border
    let border_thickness = 5.0;
    let border_color = Color::srgb(1.0, 1.0, 1.0);

    // Draw the middle dashed line
    let dash_length = 5.0;
    let dash_spacing = 5.0;
    let num_dashes = (half_window_height * 3.0 / (dash_length + dash_spacing)).ceil() as u32;
    for i in 0..num_dashes {
        let y = half_window_height * 1.5 - (i as f32 * (dash_length + dash_spacing));
        commands.spawn((
            Border,
            Sprite {
                color: border_color,
                custom_size: Some(Vec2::new(border_thickness, dash_length)),
                ..default()
            },
            Transform::from_xyz(0.0, y, 0.0),
        ));
    }
}
