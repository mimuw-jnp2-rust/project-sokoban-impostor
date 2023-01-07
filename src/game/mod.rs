use crate::{
    consts::{INITIAL_MAP, MAX_MAPS},
    labels::Labels,
    menu::reset_game_state,
    resources::RestartTimer,
    state::{DisplayState, GameState},
};
use bevy::prelude::*;
use exit::{despawn_board, handle_esc};
use maps::load_starting_map;
use victory::{delete_win, handle_box_highlight, handle_win, handle_win_click, setup_win};

pub mod display;
mod exit;
pub mod game_objects;
mod maps;
pub mod movement;
mod victory;

#[derive(Component)]
pub struct GameItem;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(DisplayState::Game(INITIAL_MAP)).with_system(set_game_state),
        );
        app.add_system_set(
            SystemSet::on_enter(DisplayState::Game(INITIAL_MAP))
                .with_system(load_starting_map.before(Labels::Display)),
        );
        for map in 0..MAX_MAPS {
            app.add_system_set(
                SystemSet::on_update(DisplayState::Game(map))
                    .with_system(handle_esc)
                    .with_system(handle_win)
                    .with_system(handle_restart)
                    .with_system(handle_box_highlight),
            );

            app.add_system_set(
                SystemSet::on_exit(DisplayState::Game(map))
                    .label(Labels::ExitGame)
                    .with_system(despawn_board),
            );

            app.add_system_set(
                SystemSet::on_pause(DisplayState::Game(map)).with_system(despawn_board),
            );
        }

        app.add_system_set(
            SystemSet::on_enter(DisplayState::Victory)
                .with_system(setup_win)
                .with_system(reset_game_state),
        )
        .add_system_set(SystemSet::on_update(DisplayState::Victory).with_system(handle_win_click))
        .add_system_set(SystemSet::on_exit(DisplayState::Victory).with_system(delete_win));

        app.add_system_set(
            SystemSet::on_update(DisplayState::Restarting)
                .with_system(reload.after(Labels::ExitGame)),
        );
    }
}

fn set_game_state(mut game_state: ResMut<State<GameState>>) {
    if game_state.current() != &GameState::Static {
        game_state
            .set(GameState::Static)
            .expect("Could not set static game state");
    }
}

fn handle_restart(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut display_state: ResMut<State<DisplayState>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        display_state
            .set(DisplayState::Restarting)
            .expect("Could not restart");
        keyboard_input.reset(KeyCode::R);
    }
}

fn reload(
    mut display_state: ResMut<State<DisplayState>>,
    mut timer: ResMut<RestartTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta()); //workaround to avoid bugs related to switching states in ::on_enter or ::on_exit
    if timer.0.finished() {
        timer.0.reset();
        display_state
            .set(DisplayState::Game(INITIAL_MAP))
            .expect("Could not reload game");
    }
}
