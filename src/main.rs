use bevy::prelude::*;

mod setup;
mod systems;

use setup::{set_window_icon, setup_camera, setup_game};
use systems::{check_new_goal, game_over, move_ball, move_paddles_with_keyboard, move_paddles_with_touch, restart_game};

#[cfg(not(target_family = "wasm"))]
use mimalloc::MiMalloc;

#[cfg(not(target_family = "wasm"))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Component)]
pub struct Paddle {
    side: Side,
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
pub struct Score {
    value: u32,
    side: Side,
}

#[derive(Component)]
pub struct Border;

pub const PADDLE_MARGIN: f32 = 50.0;
pub const PADDLE_WIDTH: f32 = 10.0;
pub const PADDLE_HEIGHT: f32 = 100.0;
pub const BALL_RADIUS: f32 = 5.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                name: Some("Pong".to_string()),
                title: "Pong".to_string(),
                #[cfg(target_family = "wasm")]
                canvas: Some("#pong".into()),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, (set_window_icon, setup_camera, setup_game))
        .add_systems(
            Update,
            (
                move_ball,
                move_paddles_with_keyboard,
                move_paddles_with_touch,
                check_new_goal,
                game_over,
                restart_game,
            ),
        )
        .run();
}

pub fn get_window_dimensions(window: &Window) -> (f32, f32) {
    (window.width() / 2.0, window.height() / 2.0)
}
