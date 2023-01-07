use bevy::prelude::*;

use crate::{resources::Board, state::CurrentMap};

pub fn handle_warp(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut current_map: ResMut<State<CurrentMap>>,
    mut board: ResMut<Board>,
) {
    if keyboard_input.pressed(KeyCode::Key1) {
        current_map
            .push(CurrentMap(Some(1)))
            .expect("Could not switch maps state");
        keyboard_input.reset(KeyCode::Key1);
        board.set_current_map(1);
    }
    if keyboard_input.pressed(KeyCode::Key0) {
        current_map.pop().expect("Could not revert maps state");
        keyboard_input.reset(KeyCode::Key0);
        board.set_current_map(0);
    }
}
