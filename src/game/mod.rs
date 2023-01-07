use crate::{
    labels::Labels,
    resources::{RestartTimer, Board},
    state::{DisplayState, GameState, CurrentMap, Move}, consts::INITIAL_MAP,
};
use bevy::prelude::*;
use exit::handle_esc;
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
            SystemSet::on_enter(DisplayState::Game)
                .with_system(load_starting_map.before(Labels::Display))
                .with_system(set_game_state)
        );
        app.add_system_set(
            SystemSet::on_update(DisplayState::Game)
                .with_system(handle_esc)
                .with_system(handle_win)
                .with_system(handle_restart)
                .with_system(handle_box_highlight.before(Labels::Movement)),
        );

        app.add_system_set(
            SystemSet::on_exit(DisplayState::Game)
                .label(Labels::ExitGame)
                .with_system(reset_game_state)
                .with_system(clear_board)
        );

        app.add_system_set(
            SystemSet::on_enter(DisplayState::Victory)
                .with_system(setup_win)
        )
        .add_system_set(SystemSet::on_update(DisplayState::Victory).with_system(handle_win_click))
        .add_system_set(SystemSet::on_exit(DisplayState::Victory).with_system(delete_win));

        app.add_system_set(
            SystemSet::on_update(DisplayState::Restarting)
                .with_system(reload.after(Labels::ExitGame))
        );
    }
}

fn set_game_state(mut game_state: ResMut<State<GameState>>, mut current_map: ResMut<State<CurrentMap>>) {
    if game_state.current() != &GameState(Some(Move::Static)) {
        game_state
            .set(GameState(Some(Move::Static)))
            .expect("Could not set static game state");
    }
    if current_map.current() != &CurrentMap(Some(INITIAL_MAP)) {
        current_map
            .set(CurrentMap(Some(INITIAL_MAP)))
            .expect("Could not set initial map");
    }
}

fn handle_restart(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut display_state: ResMut<State<DisplayState>>,
    mut current_map: ResMut<State<CurrentMap>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        display_state
            .set(DisplayState::Restarting)
            .expect("Could not restart");
        current_map
            .set(CurrentMap(None))
            .expect("Could not restart");
        keyboard_input.reset(KeyCode::R);
    }
}

fn reload(
    mut display_state: ResMut<State<DisplayState>>,
    mut current_map: ResMut<State<CurrentMap>>,
    mut timer: ResMut<RestartTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta()); //workaround to avoid bugs related to switching states in ::on_enter or ::on_exit
    if timer.0.finished() {
        timer.0.reset();
        display_state
            .set(DisplayState::Game)
            .expect("Could not reload game");
        current_map
            .set(CurrentMap(Some(INITIAL_MAP)))
        .expect("Could not reload game");
    }
}

pub fn reset_game_state(mut game_state: ResMut<State<GameState>>, mut current_map: ResMut<State<CurrentMap>>) {
    if game_state.current() != &GameState(None) {
        game_state
            .overwrite_set(GameState(None))
            .expect("Could not reset game state");
    }
    if current_map.current() != &CurrentMap(None) {
        current_map
            .overwrite_set(CurrentMap(None))
            .expect("Could not reset game state");
    }
}

pub fn clear_board(mut board: ResMut<Board>) {
    board.clear();
}