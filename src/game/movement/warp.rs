use bevy::prelude::*;

use crate::{resources::Board, state::DisplayState};

pub fn handle_warp(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<DisplayState>>,
    mut board: ResMut<Board>,
) {
    if keyboard_input.pressed(KeyCode::Key1) {
        game_state
            .push(DisplayState::Game(1))
            .expect("Could not switch maps state");
        keyboard_input.reset(KeyCode::Key1);
        board.set_current_map(1);
    }
    if keyboard_input.pressed(KeyCode::Key0) {
        game_state.pop().expect("Could not revert maps state");
        keyboard_input.reset(KeyCode::Key0);
        board.set_current_map(0);
    }
}
