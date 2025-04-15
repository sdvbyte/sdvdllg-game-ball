use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::{Audio, AudioControl};
use rand::random;

use crate::{AppState, SimulationState};

pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMIES: usize = 1;
pub const ENEMY_TIMER: f32 = 100.0;

#[derive(Component)]
pub struct Enemy {
    direction: Vec2,
}

#[derive(Resource)]
pub struct EnemyTimer {
    timer: Timer,
}

impl Default for EnemyTimer {
    fn default() -> EnemyTimer {
        EnemyTimer {
            timer: Timer::from_seconds(ENEMY_TIMER, TimerMode::Repeating),
        }
    }
}

fn enemy(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
    asset: Res<AssetServer>,
) {
    if let Ok(window) = query.get_single() {
        for _ in 0..ENEMIES {
            let window_x = random::<f32>() * window.width();
            let mut window_y = random::<f32>() * window.height();

            if window_y > 606.5265 {
                window_y -= 206.5265
            }

            commands.spawn((
                Transform::from_xyz(window_x, window_y, 0.0),
                Sprite::from_image(asset.load("sprites/ball_red_large.png")),
                Enemy {
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                },
            ));
        }
    }
}

fn enemy_run(mut query: Query<(&mut Transform, &Enemy), With<Enemy>>, time: Res<Time>) {
    for (mut transform, enemy) in query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_secs();
    }
}

fn confine_enemy(
    w: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&Transform, &mut Enemy), With<Enemy>>,
    audio: Res<Audio>,
    asset: Res<AssetServer>,
) {
    if let Ok(window) = w.get_single() {
        let half_enemy_size = ENEMY_SIZE / 2.0;
        let min_x = 0.0 + half_enemy_size;
        let max_x = window.width() - half_enemy_size;
        let min_y = 0.0 + half_enemy_size;
        let max_y = window.height() - half_enemy_size;

        for (transform, mut enemy) in query.iter_mut() {
            let mut translation = transform.translation;
            let mut change_direction = false;

            if translation.x < min_x || translation.x > max_x {
                translation.x = translation.x.clamp(min_x, max_x);
                enemy.direction.x *= -1.0;
                change_direction = true;
            }
            if translation.y < min_y || translation.y > max_y {
                translation.y = translation.y.clamp(min_y, max_y);
                enemy.direction.y *= -1.0;
                change_direction = true;
            }

            if change_direction {
                let audio_source = if random::<f32>() > 0.5 {
                    asset.load("audio/pluck_001.ogg")
                } else {
                    asset.load("audio/pluck_002.ogg")
                };
                audio.play(audio_source);
            }
        }
    }
}

fn tick_spawn_enemy_timer(mut res: ResMut<EnemyTimer>, time: Res<Time>) {
    res.timer.tick(time.delta());
}

fn spawn_enemy_over_timer(
    res: Res<EnemyTimer>,
    mut commands: Commands,
    w: Query<&Window, With<PrimaryWindow>>,
    asset: Res<AssetServer>,
) {
    if res.timer.finished() {
        if let Ok(window) = w.get_single() {
            let window_x = random::<f32>() * window.width();
            let mut window_y = random::<f32>() * window.height();

            if window_y > 606.5265 {
                window_y -= 406.5265
            }

            println!("width : {}, height : {}", window_x, window_y);

            commands.spawn((
                Transform::from_xyz(window_x, window_y, 0.0),
                Sprite::from_image(asset.load("sprites/ball_red_large.png")),
                Enemy {
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                },
            ));
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyTimer>()
            .add_systems(Startup, enemy)
            .add_systems(
                Update,
                (
                    enemy_run,
                    confine_enemy,
                    tick_spawn_enemy_timer,
                    spawn_enemy_over_timer,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
