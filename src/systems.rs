use crate::effects_system::spawn_particle_burst;
use crate::{
    get_window_dimensions, setup::setup_game, Ball, Border, HitStreak, Paddle, Score,
    ScoreCelebration, Side, Velocity, BALL_RADIUS, PADDLE_HEIGHT, PADDLE_MARGIN, PADDLE_WIDTH,
};
use bevy::{
    input::touch::TouchPhase,
    prelude::*,
    ui::{Node, UiRect, Val},
};
use rand::Rng;

const FRENZY_HIT_COUNT: u32 = 3;
const FRENZY_SPEED_MULTIPLIER: f32 = 1.5;
const KEYBOARD_PADDLE_SPEED_MULTIPLIER: f32 = 0.03;
const FRENZY_BALL_COLOR: Color = Color::srgb(1.0, 1.0, 1.0); // Bright White
const NORMAL_BALL_COLOR: Color = Color::srgb(1.0, 1.0, 0.0); // Electric Yellow

pub fn move_paddles_with_keyboard(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Paddle)>,
    windows: Query<&Window>,
) {
    let (half_window_width, half_window_height) =
        get_window_dimensions(windows.iter().next().unwrap());
    let paddle_x = half_window_width - PADDLE_MARGIN;

    for (mut transform, paddle) in query.iter_mut() {
        let mut direction = 0.0;
        if paddle.side == Side::Left {
            transform.translation.x = -paddle_x;
            if keyboard_input.pressed(KeyCode::KeyW) {
                direction += 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                direction -= 1.0;
            }
        } else {
            transform.translation.x = paddle_x;
            if keyboard_input.pressed(KeyCode::ArrowUp) {
                direction += 1.0;
            }
            if keyboard_input.pressed(KeyCode::ArrowDown) {
                direction -= 1.0;
            }
        }

        let half_paddle_height = PADDLE_HEIGHT / 2.0;
        let max_paddle_y = half_window_height - half_paddle_height;
        transform.translation.y +=
            direction * half_window_height * KEYBOARD_PADDLE_SPEED_MULTIPLIER;
        transform.translation.y = transform.translation.y.clamp(-max_paddle_y, max_paddle_y);
    }
}

#[allow(clippy::type_complexity)]
pub fn move_ball(
    mut commands: Commands, // Added Commands
    time: Res<Time>,
    mut ball_query: Query<(&mut Transform, &mut Velocity, &mut HitStreak, &mut Sprite), With<Ball>>,
    paddle_query: Query<(&Transform, &Sprite), (With<Paddle>, Without<Ball>)>, // Added &Sprite for paddle color
    windows: Query<&Window>,
) {
    let (_half_window_width, half_window_height) =
        get_window_dimensions(windows.iter().next().unwrap());
    let ball_max_y = half_window_height - BALL_RADIUS;

    // Define a maximum speed for the ball
    const MAX_BALL_SPEED_X: f32 = 800.0;
    const MAX_BALL_SPEED_Y: f32 = 800.0;
    // Define how much the ball accelerates with each paddle hit
    const BALL_ACCELERATION_MULTIPLIER: f32 = 1.1; // 10% speed increase

    for (mut ball_transform, mut ball_velocity, mut hit_streak, mut sprite) in ball_query.iter_mut()
    {
        ball_transform.translation.x += ball_velocity.x * time.delta_secs();
        ball_transform.translation.y += ball_velocity.y * time.delta_secs();

        // Bounce the ball off the top and bottom of the screen
        if ball_transform.translation.y < -ball_max_y || ball_transform.translation.y > ball_max_y {
            ball_velocity.y = -ball_velocity.y;
            // Ensure the ball doesn't get stuck outside the screen
            ball_transform.translation.y =
                ball_transform.translation.y.clamp(-ball_max_y, ball_max_y);
            // Reset hit streak if ball hits top/bottom walls
            if hit_streak.count != 0 {
                // Avoid redundant color change if already normal
                sprite.color = NORMAL_BALL_COLOR;
            }
            hit_streak.count = 0;

            // Spawn particles for wall collision
            spawn_particle_burst(
                &mut commands,
                ball_transform.translation.truncate(), // Current ball position
                Color::srgb(0.7, 0.7, 0.7),            // Gray sparks
                5,                                     // Number of particles
                0.3,                                   // Base lifetime
            );
        }

        for (paddle_transform, paddle_sprite) in paddle_query.iter() {
            // Destructure to get paddle_sprite
            let paddle_x = paddle_transform.translation.x;
            let paddle_y = paddle_transform.translation.y;
            let half_paddle_height = PADDLE_HEIGHT / 2.0;

            // Collision detection logic
            let ball_collides_with_paddle_x =
                (ball_transform.translation.x - paddle_x).abs() < PADDLE_WIDTH / 2.0 + BALL_RADIUS;

            let ball_collides_with_paddle_y = (ball_transform.translation.y
                >= paddle_y - half_paddle_height - BALL_RADIUS)
                && (ball_transform.translation.y <= paddle_y + half_paddle_height + BALL_RADIUS);

            // Check if the ball is moving towards the paddle
            let ball_moving_towards_paddle = (ball_velocity.x > 0.0 && paddle_x > 0.0) || // Ball moving right, right paddle
                (ball_velocity.x < 0.0 && paddle_x < 0.0); // Ball moving left, left paddle

            if ball_collides_with_paddle_x
                && ball_collides_with_paddle_y
                && ball_moving_towards_paddle
            {
                // Prevent ball from passing through paddle by adjusting its position
                if ball_velocity.x > 0.0 {
                    // Moving right
                    ball_transform.translation.x = paddle_x - PADDLE_WIDTH / 2.0 - BALL_RADIUS;
                } else {
                    // Moving left
                    ball_transform.translation.x = paddle_x + PADDLE_WIDTH / 2.0 + BALL_RADIUS;
                }

                // Reverse and accelerate ball's x velocity
                ball_velocity.x *= -BALL_ACCELERATION_MULTIPLIER;
                // Cap the ball's x speed
                ball_velocity.x = ball_velocity.x.clamp(-MAX_BALL_SPEED_X, MAX_BALL_SPEED_X);

                // Calculate the offset from the center of the paddle
                // Offset is between -1.0 (top of paddle) and 1.0 (bottom of paddle)
                let offset = (paddle_y - ball_transform.translation.y) / half_paddle_height;

                // Modify y velocity based on where the ball hit the paddle
                // Max y deflection angle (e.g. 60 degrees), converting to a multiplier for y velocity
                // A higher offset results in a larger change in y velocity.
                // The current y velocity is also taken into account and slightly amplified.
                let new_y_velocity = ball_velocity.y * 0.5 - offset * (MAX_BALL_SPEED_Y * 0.75);
                ball_velocity.y = new_y_velocity.clamp(-MAX_BALL_SPEED_Y, MAX_BALL_SPEED_Y);

                // Accelerate ball's y velocity (less than x to maintain some control)
                // ball_velocity.y *= BALL_ACCELERATION_MULTIPLIER * 0.9; // slightly less acceleration for y
                // Cap the ball's y speed
                // ball_velocity.y = ball_velocity.y.clamp(-MAX_BALL_SPEED_Y, MAX_BALL_SPEED_Y);

                // Handle Hit Streak for Frenzy Ball
                hit_streak.count += 1;
                if hit_streak.count >= FRENZY_HIT_COUNT {
                    ball_velocity.x *= FRENZY_SPEED_MULTIPLIER;
                    ball_velocity.y *= FRENZY_SPEED_MULTIPLIER;
                    // Cap speeds after frenzy boost
                    ball_velocity.x = ball_velocity.x.clamp(-MAX_BALL_SPEED_X, MAX_BALL_SPEED_X);
                    ball_velocity.y = ball_velocity.y.clamp(-MAX_BALL_SPEED_Y, MAX_BALL_SPEED_Y);

                    sprite.color = FRENZY_BALL_COLOR; // Set frenzy color
                    hit_streak.count = 0; // Reset streak after frenzy activates (for burst effect)
                                          // Color will remain FRENZY_BALL_COLOR until next wall hit or goal
                } else if hit_streak.count == 0 && sprite.color == FRENZY_BALL_COLOR {
                    // This case handles when frenzy was just activated (count became 0)
                    // and ensures normal color if no new frenzy is immediately triggered by this same hit (which is impossible)
                    // More simply: if streak is 0, color should be normal.
                    // This 'else if' is mostly for clarity; wall/goal resets handle long-term.
                    // However, this ensures if frenzy activates, and then it's a normal hit, it reverts.
                    // This will be handled by the wall/goal reset mostly.
                    // The primary place to reset to NORMAL_BALL_COLOR is when hit_streak.count becomes 0.
                }
                // If hit_streak.count > 0 but < FRENZY_HIT_COUNT, color should be normal.
                if hit_streak.count > 0
                    && hit_streak.count < FRENZY_HIT_COUNT
                    && sprite.color == FRENZY_BALL_COLOR
                {
                    sprite.color = NORMAL_BALL_COLOR;
                }

                // Spawn particles for paddle collision
                spawn_particle_burst(
                    &mut commands,
                    ball_transform.translation.truncate(), // Current ball position
                    paddle_sprite.color,                   // Use paddle's color
                    7,                                     // Number of particles
                    0.4,                                   // Base lifetime
                );
            }
        }
    }
}

// Constants CELEBRATION_DURATION_SECS and NORMAL_SCORE_COLOR were here,
// but CELEBRATION_TEXT_COLOR is what's used by manage_score_celebration.
// NORMAL_SCORE_COLOR is implicitly the one set during setup.

pub fn check_new_goal(
    mut commands: Commands,
    mut score_display_query: Query<(Entity, &mut Score, &mut Text, &TextColor)>, // Changed to &mut Text
    mut ball_query: Query<(&mut Transform, &mut Velocity, &mut HitStreak, &mut Sprite), With<Ball>>,
    windows: Query<&Window>,
) {
    let (half_window_width, half_window_height) =
        get_window_dimensions(windows.iter().next().unwrap());
    let ball_limit = half_window_width + PADDLE_MARGIN; // How far the ball has to go to score

    for (mut ball_transform, mut ball_velocity, mut hit_streak, mut ball_sprite) in
        ball_query.iter_mut()
    {
        let mut player_scored: Option<Side> = None;

        if ball_transform.translation.x < -ball_limit {
            player_scored = Some(Side::Right);
        } else if ball_transform.translation.x > ball_limit {
            player_scored = Some(Side::Left);
        }

        if let Some(actual_winner_side) = player_scored {
            // Update score and trigger celebration
            for (text_entity, mut score_component, mut text, text_color) in
                score_display_query.iter_mut()
            {
                // text_object is &mut Text
                if score_component.side == actual_winner_side {
                    score_component.value += 1;

                    // Update the displayed text value directly
                    text.0 = score_component.value.to_string();

                    // Add ScoreCelebration component to the text_entity
                    let original_color = text_color.0; // Get current color before changing
                    commands.entity(text_entity).insert(ScoreCelebration {
                        timer: Timer::from_seconds(
                            crate::effects_system::CELEBRATION_DURATION_SECS,
                            TimerMode::Once,
                        ),
                        original_color,
                        scored_side: actual_winner_side,
                    });
                    // The manage_score_celebration system will now handle changing to CELEBRATION_TEXT_COLOR
                    break;
                }
            }

            // Reset ball position to center
            ball_transform.translation = Vec3::new(0.0, 0.0, 0.1);

            // Determine serve direction (towards the player who didn't score)
            let serve_direction_x = match actual_winner_side {
                Side::Left => 1.0,   // Serve to the right (towards right player)
                Side::Right => -1.0, // Serve to the left (towards left player)
            };

            let initial_ball_speed_x = half_window_width / 2.0;
            let initial_ball_speed_y_range = half_window_height / 2.0;

            let mut rng = rand::thread_rng();
            ball_velocity.x = serve_direction_x * initial_ball_speed_x;
            // Random initial y speed for variety, but less than x to make it receivable
            ball_velocity.y =
                rng.gen_range(-initial_ball_speed_y_range * 0.5..=initial_ball_speed_y_range * 0.5);

            // Reset hit streak on goal
            hit_streak.count = 0;
            // Reset ball color on goal
            ball_sprite.color = NORMAL_BALL_COLOR;
        }
    }
}

//Stop ball from moving, reset score, despawn paddles and ball, display game over text, then totally reset
pub fn game_over(
    score: Query<&Score>,
    mut commands: Commands,
    text_query: Query<Entity, With<Text>>,
    paddle_query: Query<Entity, With<Paddle>>,
    ball_query: Query<Entity, With<Ball>>,
    border_query: Query<Entity, With<Border>>,
    windows: Query<&Window>,
) {
    let (half_window_width, half_window_height) =
        get_window_dimensions(windows.iter().next().unwrap());
    for score in score.iter() {
        if score.value == 10 {
            for entity in text_query.iter() {
                commands.entity(entity).despawn();
            }
            for entity in paddle_query.iter() {
                commands.entity(entity).despawn();
            }
            for entity in ball_query.iter() {
                commands.entity(entity).despawn();
            }
            for entity in border_query.iter() {
                commands.entity(entity).despawn();
            }

            commands.spawn((
                Text::new(format!("P{} wins!", u8::from(score.side == Side::Left) + 1)),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        left: Val::Px(half_window_width - 200.0),
                        right: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        top: Val::Px(half_window_height - 100.0),
                    },
                    ..Default::default()
                },
            ));

            commands.spawn((
                Text::new("Press R to restart".to_string()),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        left: Val::Px(half_window_width - 200.0),
                        right: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        top: Val::Px(half_window_height - 50.0),
                    },
                    ..Default::default()
                },
            ));
        }
    }
}

//Unavoidable when using Bevy queries
#[allow(clippy::too_many_arguments)]
pub fn restart_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touch_input: Res<Touches>,
    mut commands: Commands,
    score_query: Query<&Score>,
    score_entties: Query<Entity, With<Score>>,
    text_entities: Query<Entity, With<Text>>,
    paddle_entities: Query<Entity, With<Paddle>>,
    ball_entities: Query<Entity, With<Ball>>,
    windows: Query<&Window>,
) {
    let entities = score_entties
        .iter()
        .chain(text_entities.iter())
        .chain(paddle_entities.iter())
        .chain(ball_entities.iter());

    let restart_triggered = {
        keyboard_input.just_pressed(KeyCode::KeyR)
            || (touch_input.any_just_pressed() && score_query.iter().any(|score| score.value == 10))
    };

    if restart_triggered {
        for entity in entities.into_iter() {
            commands.entity(entity).despawn();
        }
        setup_game(commands, windows);
    }
}

pub fn move_paddles_with_touch(
    mut touch_input: EventReader<TouchInput>,
    mut paddles_query: Query<(&Paddle, &mut Transform)>,
    windows: Query<&Window>,
) {
    let window = windows.iter().next().unwrap();
    let (_half_window_width, half_window_height) = get_window_dimensions(window);

    // Calculate paddle clamping limits
    let half_paddle_height = PADDLE_HEIGHT / 2.0;
    let max_paddle_y = half_window_height - half_paddle_height;

    for touch_event in touch_input.read() {
        // We only care about moved touches for paddle control
        if touch_event.phase != TouchPhase::Moved {
            continue;
        }

        // Convert touch position (origin top-left) to Bevy world coordinates (origin center)
        // Touch Y increases downwards, Bevy Y increases upwards.
        let touch_y_bevy = half_window_height - touch_event.position.y;

        // Determine which paddle to move based on touch X position
        // (0,0) is top-left for touch_event.position.x, window.width() is right edge
        if touch_event.position.x < window.width() / 2.0 {
            // Touch is on the left half of the screen, move left paddle
            if let Some((_paddle, mut transform)) =
                paddles_query.iter_mut().find(|(p, _)| p.side == Side::Left)
            {
                transform.translation.y = touch_y_bevy.clamp(-max_paddle_y, max_paddle_y);
            }
        } else {
            // Touch is on the right half of the screen, move right paddle
            if let Some((_paddle, mut transform)) = paddles_query
                .iter_mut()
                .find(|(p, _)| p.side == Side::Right)
            {
                transform.translation.y = touch_y_bevy.clamp(-max_paddle_y, max_paddle_y);
            }
        }
    }
}
