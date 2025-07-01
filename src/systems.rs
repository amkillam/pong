use crate::{
    get_window_dimensions, setup::setup_game, Ball, Border, Paddle, Score, Side, Velocity,
    BALL_RADIUS, PADDLE_HEIGHT, PADDLE_MARGIN, PADDLE_WIDTH,
};
use bevy::{prelude::*, input::touch::TouchPhase, ui::{Node, Val, UiRect}};
use rand::Rng;

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
        transform.translation.y += direction * half_window_height * 0.03;
        transform.translation.y = transform.translation.y.clamp(-max_paddle_y, max_paddle_y);
    }
}

pub fn move_ball(
    time: Res<Time>,
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    windows: Query<&Window>,
) {
    let (_half_window_width, half_window_height) =
        get_window_dimensions(windows.iter().next().unwrap());
    let ball_max_y = half_window_height - BALL_RADIUS;

    for (mut ball_transform, mut ball_velocity) in ball_query.iter_mut() {
        ball_transform.translation.x += ball_velocity.x * time.delta_secs();
        ball_transform.translation.y += ball_velocity.y * time.delta_secs();

        // Bounce the ball off the top and bottom of the screen
        if ball_transform.translation.y < -ball_max_y || ball_transform.translation.y > ball_max_y {
            ball_velocity.y = -ball_velocity.y;
        }

        for paddle_transform in paddle_query.iter() {
            // Bounce the ball off the paddles and accelerate
            let is_collision = (ball_transform.translation.x - paddle_transform.translation.x)
                .abs()
                < PADDLE_WIDTH
                && (ball_velocity.x.is_sign_negative()
                    == paddle_transform.translation.x.is_sign_negative())
                && ((ball_transform.translation.y - paddle_transform.translation.y).abs() < 50.0);
            if is_collision {
                ball_velocity.x *= -1.1;
                ball_velocity.y *= 1.1;
            }
        }
    }
}

pub fn check_new_goal(
    mut score_display: Query<(&mut Score, Entity), With<Text>>,
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    windows: Query<&Window>,
    mut text_writer: TextUiWriter,
) {
    let (half_window_width, half_window_height) =
        get_window_dimensions(windows.iter().next().unwrap());
    let ball_limit = half_window_width + PADDLE_MARGIN;

    for (mut ball_transform, mut velocity) in ball_query.iter_mut() {
        if ball_transform.translation.x < -ball_limit || ball_transform.translation.x > ball_limit {
            // Increment the score based on which side the ball went past
            if ball_transform.translation.x < -ball_limit {
                for (mut score, text_entity) in score_display.iter_mut() {
                    if score.side == Side::Right {
                        score.value += 1;
                        *text_writer.text(text_entity, 0) = score.value.to_string();
                    }
                }
            } else if ball_transform.translation.x > ball_limit {
                for (mut score, text_entity) in score_display.iter_mut() {
                    if score.side == Side::Left {
                        score.value += 1;
                        *text_writer.text(text_entity, 0) = score.value.to_string();
                    }
                }
            }
            // Reset the ball to the center with a new random velocity
            let quarter_window_width = half_window_width / 2.0;
            let quarter_window_height = half_window_height / 2.0;

            let mut rng = rand::thread_rng();
            velocity.x = if rng.gen() {
                quarter_window_width
            } else {
                -quarter_window_width
            };
            velocity.y = rng.gen_range(-quarter_window_height..=quarter_window_height);
            ball_transform.translation = Vec3::new(0.0, 0.0, 0.1);
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
                Text::new(
                    format!("P{} wins!", u8::from(score.side == Side::Left) + 1),
                ),
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
                Text::new(
                    "Press R to restart".to_string(),
                ),
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
    let half_window_width = window.width() / 2.0;
    let window_height = window.height();
    for finger in touch_input.read() {
        let mut adjusted_finger_y = match finger.phase {
            TouchPhase::Moved => match finger.position.y {
                y if y < 0.0 => -(y * 2.0),
                y => -(y / 2.0),
            },
            _ => continue,
        };
        if adjusted_finger_y > window_height {
            adjusted_finger_y = window_height;
        }
        if finger.position.x < half_window_width {
            let mut left_paddle = paddles_query
                .iter_mut()
                .find(|(paddle, _)| paddle.side == Side::Left)
                .unwrap();

            left_paddle.1.translation.y = adjusted_finger_y
        } else {
            let mut right_paddle = paddles_query
                .iter_mut()
                .find(|(paddle, _)| paddle.side == Side::Right)
                .unwrap();
            right_paddle.1.translation.y = adjusted_finger_y
        }
    }
}