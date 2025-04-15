use bevy::{
    prelude::*, render::view::screenshot::ScreenshotToScreenPipeline, window::PrimaryWindow,
};
use bevy_kira_audio::{Audio, AudioControl};
use rand::random;

use crate::{
    enemy::{Enemy, ENEMY_SIZE},
    star::{Star, STAR_SIZE},
};

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

#[derive(Resource)]
pub struct Kill {
    value: u32,
}

impl Default for Kill {
    fn default() -> Kill {
        Kill { value: 0 }
    }
}

#[derive(Component)]
pub struct Player {}

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
}

fn player(mut commands: Commands, w: Query<&Window, With<PrimaryWindow>>, asset: Res<AssetServer>) {
    if let Ok(window) = w.get_single() {
        let window_x = window.width() / 2.0;
        let window_y = window.height() / 2.0;

        commands.spawn((
            Transform::from_xyz(window_x, window_y, 0.0),
            Sprite::from_image(asset.load("sprites/ball_blue_large.png")),
            Player {},
        ));

        commands.spawn((
            Transform::from_xyz(window_x, window_y, 0.0),
            Camera2d::default(),
        ));
    }
}

fn player_run(
    keyinput: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyinput.pressed(KeyCode::ArrowLeft) {
            direction = Vec3::new(-2.0, 0.0, 0.0);
        }
        if keyinput.pressed(KeyCode::ArrowRight) {
            direction = Vec3::new(2.0, 0.0, 0.0);
        }
        if keyinput.pressed(KeyCode::ArrowUp) {
            direction = Vec3::new(0.0, 2.0, 0.0);
        }
        if keyinput.pressed(KeyCode::ArrowDown) {
            direction = Vec3::new(0.0, -2.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

fn confine_player(
    w: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<Player>>,
    audio: Res<Audio>,
    asset: Res<AssetServer>,
) {
    if let Ok(window) = w.get_single() {
        let half_player_size = PLAYER_SIZE / 2.0;

        let min_x = 0.0 + half_player_size;
        let max_x = window.width() - half_player_size;

        let min_y = 0.0 + half_player_size;
        let max_y = window.height() - half_player_size;

        if let Ok(mut transform) = query.get_single_mut() {
            let mut translation = transform.translation;
            let mut change_direction = false;

            if translation.x < min_x || translation.x > max_x {
                change_direction = true;
            }

            if translation.y < min_y || translation.y > max_y {
                change_direction = true;
            }

            if translation.x < min_x {
                translation.x = min_x
            }

            if translation.x > max_x {
                translation.x = max_x
            }

            if translation.y < min_y {
                translation.y = min_y
            }

            if translation.y > max_y {
                translation.y = max_y
            }

            if change_direction {
                let audio_source = if random::<f32>() > 0.5 {
                    asset.load("audio/pluck_001.ogg")
                } else {
                    asset.load("audio/pluck_002.ogg")
                };
                audio.play(audio_source);
            }

            transform.translation = translation;
        }
    }
}

fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    audio: Res<Audio>,
    asset: Res<AssetServer>,
    score: Res<Kill>,
) {
    if let Ok((entity, transform_player)) = player_query.get_single() {
        for transform in enemy_query.iter() {
            let distance = transform_player.translation.distance(transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                audio.play(asset.load("audio/explosionCrunch_000.ogg"));
                game_over_event_writer.send(GameOver { score: score.value });
                commands.entity(entity).despawn();
            }
        }
    }
}

fn player_hit_star(
    mut commands: Commands,
    mut query: Query<&Transform, With<Player>>,
    mut star_query: Query<(Entity, &Transform), With<Star>>,
    audio: Res<Audio>,
    asset: Res<AssetServer>,
    mut kill: ResMut<Kill>,
) {
    if let Ok(transform) = query.get_single_mut() {
        for (entity, transform_star) in star_query.iter_mut() {
            let distance = transform.translation.distance(transform_star.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;
            if distance < player_radius + star_radius {
                kill.value += 1;
                audio.play(asset.load("audio/laserLarge_000.ogg"));
                commands.entity(entity).despawn();
            }
        }
    }
}

fn show_kill_star(res: Res<Kill>) {
    if res.is_changed() {
        println!("Kill star : {}", res.value.to_string());
    }
}

fn game_over_reader(mut res: EventReader<GameOver>) {
    for event in res.read() {
        println!("Hight value {}", event.score);
    }
}

#[derive(SystemSet, PartialEq, Eq, Clone, Hash, Debug)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Kill>()
            .configure_sets(
                Update,
                PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement),
            )
            .add_event::<GameOver>()
            .add_systems(Startup, player)
            .add_systems(
                Update,
                (
                    player_run.in_set(PlayerSystemSet::Movement),
                    confine_player.in_set(PlayerSystemSet::Confinement),
                    enemy_hit_player,
                    player_hit_star,
                    show_kill_star,
                    game_over_reader,
                ),
            );
    }
}
