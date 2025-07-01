use bevy::prelude::*;

mod effects_system;
mod setup;
mod systems;

use effects_system::EffectsPlugin;
use setup::{set_window_icon, setup_camera, setup_game};
use systems::{
    check_new_goal, game_over, move_ball, move_paddles_with_keyboard, move_paddles_with_touch,
    restart_game,
};

#[cfg(not(target_family = "wasm"))]
use mimalloc::MiMalloc;

#[cfg(not(target_family = "wasm"))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Component)]
pub struct Paddle {
    pub side: Side,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

#[derive(Component)]
pub struct Ball;

#[derive(Component, Default)]
pub struct HitStreak {
    pub count: u32,
}

#[derive(Component)]
pub struct Particle {
    // Ensure this is public if accessed from particle_system.rs
    pub lifetime: Timer,
}

#[derive(Component)]
pub struct ScoreCelebration {
    pub timer: Timer,
    pub original_color: Color,
    // We might not need original_font_size if we just flash color
    // pub original_font_size: f32,
    pub scored_side: Side, // To know which score text this belongs to
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Clone, Copy)]
pub struct Score {
    pub value: u32,
    pub side: Side,
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
                title: "Pong".to_string(),
                #[cfg(target_family = "wasm")]
                canvas: Some("#pong".into()),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.3))) // Deep Purple background
        .add_plugins(EffectsPlugin) // Changed from ParticlePlugin
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
