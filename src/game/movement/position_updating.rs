use bevy::prelude::*;

use crate::{
    resources::{Board, MovementData},
    state::GameState,
};

use super::events::MoveEvent;

pub fn handle_move(
    mut reader: EventReader<MoveEvent>,
    mut board: ResMut<Board>,
    mut movement_data: ResMut<MovementData>,
    mut app_state: ResMut<State<GameState>>,
) {
    let mut was_movement = false;
    for event in reader.iter() {
        was_movement = true;
        let position = event.position;
        let dir = event.direction;
        board.move_object(position, dir);
        movement_data.moved_positions.push(position);
        movement_data.direction = Some(dir);
    }
    if was_movement {
        app_state
            .push(GameState::Moving)
            .expect("Could not switch states");
    }
}
