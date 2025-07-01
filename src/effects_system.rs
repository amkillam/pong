use crate::{Particle, ScoreCelebration};
use bevy::prelude::*; // Assuming these are pub in main.rs or lib.rs

// Define colors for celebration
const CELEBRATION_TEXT_COLOR: Color = Color::srgb(1.0, 1.0, 1.0); // Bright white
pub const CELEBRATION_DURATION_SECS: f32 = 0.5; // Shorter flash

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (manage_particles, manage_score_celebration));
    }
}

pub fn manage_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &mut Sprite)>,
) {
    for (entity, mut particle, mut sprite) in query.iter_mut() {
        particle.lifetime.tick(time.delta());
        if particle.lifetime.just_finished() {
            commands.entity(entity).despawn();
        } else {
            let remaining_ratio = 1.0 - particle.lifetime.fraction();
            sprite.color.set_alpha(remaining_ratio);
        }
    }
}

pub fn spawn_particle_burst(
    commands: &mut Commands,
    position: Vec2,
    color: Color,
    count: u32,
    base_lifetime_secs: f32,
) {
    for _ in 0..count {
        let lifetime_variation = rand::random::<f32>() * 0.2 - 0.1;
        let lifetime = (base_lifetime_secs + lifetime_variation).max(0.1);

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(3.0, 3.0)),
                ..default()
            },
            Transform::from_xyz(position.x, position.y, 0.2),
            Particle {
                lifetime: Timer::from_seconds(lifetime, TimerMode::Once),
            },
        ));
    }
}

pub fn manage_score_celebration(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ScoreCelebration, &mut TextColor)>,
) {
    for (entity, mut celebration, mut text_color) in query.iter_mut() {
        celebration.timer.tick(time.delta());

        if celebration.timer.just_finished() {
            // Reset to original color
            *text_color = TextColor(celebration.original_color);
            commands.entity(entity).remove::<ScoreCelebration>();
        } else {
            // Ensure celebration color is set if not already (it's set on spawn)
            if text_color.0 != CELEBRATION_TEXT_COLOR {
                *text_color = TextColor(CELEBRATION_TEXT_COLOR);
            }
        }
    }
}
