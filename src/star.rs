use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::{Audio, AudioControl};
use rand::random;

pub const STAR_SPEED: f32 = 80.0;
pub const STAR_SIZE: f32 = 64.0;
pub const STARS: usize = 1;
pub const STAR_TIMER: f32 = 100.0;

#[derive(Component)]
pub struct Star {
    direction: Vec2,
}

#[derive(Resource)]
pub struct StarTimer {
    timer: Timer,
}

impl Default for StarTimer {
    fn default() -> StarTimer {
        StarTimer {
            timer: Timer::from_seconds(STAR_TIMER, TimerMode::Repeating),
        }
    }
}

fn star(mut commands: Commands, w: Query<&Window, With<PrimaryWindow>>, asset: Res<AssetServer>) {
    if let Ok(window) = w.get_single() {
        for _ in 0..STARS {
            let window_x = random::<f32>() * window.width();
            let mut window_y = random::<f32>() * window.height();

            if window_y > 606.5265 {
                window_y -= 406.5265
            }

            commands.spawn((
                Transform::from_xyz(window_x, window_y, 0.0),
                Sprite::from_image(asset.load("sprites/star.png")),
                Star {
                    direction: Vec2::new(random::<f32>(), random::<f32>()),
                },
            ));
        }
    }
}

fn star_run(mut query: Query<(&mut Transform, &Star), With<Star>>, time: Res<Time>) {
    for (mut transform, star) in query.iter_mut() {
        let direction = Vec3::new(star.direction.x, star.direction.y, 0.0);
        transform.translation += direction * STAR_SPEED * time.delta_secs();
    }
}

fn confine_star(
    w: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&Transform, &mut Star), With<Star>>,
    audio: Res<Audio>,
    asset: Res<AssetServer>,
) {
    if let Ok(window) = w.get_single() {
        let half_star_size = STAR_SIZE / 2.0;
        let min_x = 0.0 + half_star_size;
        let max_x = window.width() - half_star_size;
        let min_y = 0.0 + half_star_size;
        let max_y = window.height() - half_star_size;
        for (transform, mut star) in query.iter_mut() {
            let mut translation = transform.translation;
            let mut change_direction = false;

            if translation.x < min_x || translation.x > max_x {
                star.direction.x *= -1.0;
                translation.x = translation.x.clamp(min_x, max_x);
                change_direction = true;
            }

            if translation.y < min_y || translation.y > max_y {
                star.direction.y *= -1.0;
                translation.y = translation.y.clamp(min_y, max_y);
                change_direction = true;
            }

            if change_direction {
                let audio_source = if random::<f32>() > 0.5 {
                    asset.load("audio/pluck_001.ogg")
                } else {
                    asset.load("audio/pluck_001.ogg")
                };
                audio.play(audio_source);
            }
        }
    }
}

fn tick_spawn_star_timer(mut res: ResMut<StarTimer>, time: Res<Time>) {
    res.timer.tick(time.delta());
}

fn spawn_star_over_timer(
    mut commands: Commands,
    w: Query<&Window, With<PrimaryWindow>>,
    timer: Res<StarTimer>,
    asset: Res<AssetServer>,
) {
    if timer.timer.finished() {
        if let Ok(window) = w.get_single() {
            let window_x = random::<f32>() * window.width();
            let window_y = random::<f32>() * window.height();

            commands.spawn((
                Transform::from_xyz(window_x, window_y, 0.0),
                Sprite::from_image(asset.load("sprites/star.png")),
                Star {
                    direction: Vec2::new(random::<f32>(), random::<f32>()),
                },
            ));
        }
    }
}

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarTimer>()
            .add_systems(Startup, star)
            .add_systems(
                Update,
                (
                    star_run,
                    confine_star,
                    tick_spawn_star_timer,
                    spawn_star_over_timer,
                ),
            );
    }
}
