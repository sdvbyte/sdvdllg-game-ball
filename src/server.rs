use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use star::StarPlugin;

mod enemy;
mod player;
mod star;

#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}

fn toggle_simulations(
    keyinput: Res<ButtonInput<KeyCode>>,
    state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    if keyinput.just_pressed(KeyCode::Space) {
        if *state.get() == SimulationState::Running {
            next_state.set(SimulationState::Paused);
            println!("Simulations Paused.");
        } else {
            next_state.set(SimulationState::Running);
            println!("Simulations Running.");
        }
    }
}

fn transition_to_main_menu(
    keyinput: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyinput.just_pressed(KeyCode::KeyG) {
        next_state.set(AppState::Game);
        println!("Entered AppState::Game");
    }
    if keyinput.just_pressed(KeyCode::KeyM) {
        next_state.set(AppState::MainMenu);
        println!("Entered AppState::MainMenu");
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AudioPlugin::default()))
        .init_state::<AppState>()
        .init_state::<SimulationState>()
        .add_plugins(EnemyPlugin)
        .add_plugins(StarPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Update, transition_to_main_menu)
        .add_systems(Update, toggle_simulations.run_if(in_state(AppState::Game)))
        .run();
}
