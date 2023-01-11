use crate::{
    consts::INITIAL_MAP,
    labels::Labels,
    resources::{Board, BoardStates},
    state::{CurrentMap, DisplayState, GameState, Move},
};
use bevy::prelude::*;
use exit::handle_esc;
use maps::load_starting_map;
use victory::{delete_win, handle_win, handle_win_click, setup_win};

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
                .with_system(load_starting_map)
                .with_system(set_game_state),
        );
        app.add_system_set(
            SystemSet::on_update(DisplayState::Game)
                .with_system(handle_esc)
                .with_system(handle_win),
        );

        app.add_system_set(
            SystemSet::on_exit(DisplayState::Game)
                .label(Labels::ExitGame)
                .with_system(reset_game_state)
                .with_system(clear_board),
        );

        app.add_system_set(SystemSet::on_enter(DisplayState::Victory).with_system(setup_win))
            .add_system_set(
                SystemSet::on_update(DisplayState::Victory).with_system(handle_win_click),
            )
            .add_system_set(SystemSet::on_exit(DisplayState::Victory).with_system(delete_win));

        app.add_system_set(
            SystemSet::on_update(GameState(Some(Move::Static)))
                .with_system(handle_restart)
                .with_system(handle_undo),
        );
    }
}

fn set_game_state(
    mut game_state: ResMut<State<GameState>>,
    mut current_map: ResMut<State<CurrentMap>>,
) {
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

pub fn reset_game_state(
    mut game_state: ResMut<State<GameState>>,
    mut current_map: ResMut<State<CurrentMap>>,
) {
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

fn handle_restart(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut current_map: ResMut<State<CurrentMap>>,
    mut boards: ResMut<BoardStates>,
    mut board: ResMut<Board>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        if !boards.boards.is_empty() {
            *board = boards.boards[0].clone();
            boards.boards.clear();
        }
        if current_map.current() != &CurrentMap(Some(INITIAL_MAP)) {
            current_map
                .set(CurrentMap(Some(INITIAL_MAP)))
                .expect("Could not restart");
        }
        keyboard_input.reset(KeyCode::R);
    }
}

pub fn handle_undo(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut current_map: ResMut<State<CurrentMap>>,
    mut boards: ResMut<BoardStates>,
    mut board: ResMut<Board>,
) {
    if keyboard_input.just_pressed(KeyCode::U) && !boards.boards.is_empty() {
        *board = boards.boards.pop().expect("Could not get last move");
        let new_map = board.get_current_map();
        if let CurrentMap(Some(state_map)) = current_map.current() {
            if *state_map != new_map {
                current_map
                    .set(CurrentMap(Some(new_map)))
                    .expect("Could not undo map state");
            }
        }
        keyboard_input.reset(KeyCode::U);
    }
}

pub fn clear_board(mut board: ResMut<Board>, mut boards: ResMut<BoardStates>) {
    board.clear();
    boards.boards.clear();
}
