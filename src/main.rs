use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
struct Paddle {
    side: Side,
}

#[derive(PartialEq, Eq)]
enum Side {
    Left,
    Right,
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Score {
    value: u32,
    side: Side,
}

#[derive(Component)]
struct Border;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                name: Some("Pong".to_string()),
                title: "Pong".to_string(),

                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, (setup_camera, setup_game))
        .add_systems(
            Update,
            (
                move_ball,
                move_paddles,
                check_new_goal,
                game_over,
                restart_game,
            ),
        )
        .run();
}

// Spawns the camera that draws UI
fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

fn get_window_dimensions(window: &Window) -> (f32, f32) {
    (window.width() / 2.0, window.height() / 2.0)
}

fn restart_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    score: Query<Entity, With<Score>>,
    text_query: Query<Entity, With<Text>>,
    paddle_query: Query<Entity, With<Paddle>>,
    ball_query: Query<Entity, With<Ball>>,
    windows: Query<&Window>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        for entity in text_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in paddle_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in ball_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in score.iter() {
            commands.entity(entity).despawn();
        }
        setup_game(commands, windows);
    }
}

const PADDLE_MARGIN: f32 = 50.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 100.0;
const BALL_RADIUS: f32 = 5.0;

fn setup_game(mut commands: Commands, windows: Query<&Window>) {
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

fn move_paddles(
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
        transform.translation.y += direction * half_window_height * 0.02;
        transform.translation.y = transform.translation.y.clamp(-max_paddle_y, max_paddle_y);
    }
}

fn move_ball(
    time: Res<Time>,
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    windows: Query<&Window>,
) {
    let (_half_window_width, half_window_height) =
        get_window_dimensions(windows.iter().next().unwrap());
    let ball_max_y = half_window_height - BALL_RADIUS;

    for (mut ball_transform, mut ball_velocity) in ball_query.iter_mut() {
        ball_transform.translation.x += ball_velocity.x * time.delta_seconds();
        ball_transform.translation.y += ball_velocity.y * time.delta_seconds();

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

fn check_new_goal(
    mut score_display: Query<(&mut Score, &mut Text)>,
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    windows: Query<&Window>,
) {
    let (half_window_width, half_window_height) =
        get_window_dimensions(windows.iter().next().unwrap());
    let ball_limit = half_window_width + PADDLE_MARGIN;

    for (mut ball_transform, mut velocity) in ball_query.iter_mut() {
        if ball_transform.translation.x < -ball_limit || ball_transform.translation.x > ball_limit {
            // Increment the score based on which side the ball went past
            if ball_transform.translation.x < -ball_limit {
                for (mut score, mut text) in score_display.iter_mut() {
                    if score.side == Side::Right {
                        score.value += 1;
                        text.sections[0].value = score.value.to_string();
                    }
                }
            } else if ball_transform.translation.x > ball_limit {
                for (mut score, mut text) in score_display.iter_mut() {
                    if score.side == Side::Left {
                        score.value += 1;
                        text.sections[0].value = score.value.to_string();
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
fn game_over(
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

            let text_style_style = Style {
                position_type: PositionType::Absolute,
                margin: UiRect {
                    left: Val::Px(half_window_width - 200.0),
                    right: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    top: Val::Px(half_window_height - 100.0),
                },
                ..Default::default()
            };

            let text_style = TextStyle {
                font: Handle::default(),
                font_size: 40.0,
                color: Color::rgb(1.0, 1.0, 1.0),
            };

            commands.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: format!("P{} wins!", u8::from(score.side == Side::Left) + 1,),
                            style: text_style.clone(),
                        },
                        TextSection {
                            value: "\r\rPress R to restart".to_string(),
                            style: text_style,
                        },
                    ],
                    ..Default::default()
                },
                style: text_style_style,
                ..Default::default()
            });
        }
    }
}
