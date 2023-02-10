use bevy::prelude::*;

use crate::game::game_objects::{Direction, *};
use crate::game::resources::{Board, BoardStates};
use crate::state::{GameState, Move};

use super::events::ExitedFloorEvent;

pub fn handle_keypress(
    keyboard_input: ResMut<Input<KeyCode>>,
    board: Res<Board>,
    mut writer: EventWriter<ExitedFloorEvent>,
    mut app_state: ResMut<State<GameState>>,
    mut board_states: ResMut<BoardStates>,
) {
    let direction = if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
        Direction::Up
    } else if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
        Direction::Down
    } else if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
        Direction::Left
    } else if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        Direction::Right
    } else {
        return;
    };
    let mut position = board.get_player_position();
    let mut positions = Vec::new();
    let mut next_position = board.get_next_position_for_move(position, direction);
    positions.push(position);
    //we iterate to see if there is an empty space after some boxes
    while board.get_object_type(next_position) == GameObject::Box {
        position = next_position;
        positions.push(position);
        next_position = board.get_next_position_for_move(next_position, direction);
    }
    positions.reverse(); //we want to move the last box as first, so that they don't overlap
    let object_blocking = board.get_object_type(next_position);
    if object_blocking == GameObject::Empty {
        board_states.boards.push(board.clone());
        for position in positions {
            writer.send(ExitedFloorEvent {
                floor: board.get_floor_type(position),
                position,
                direction,
                object: board.get_object_type(position),
            });
        }
        app_state
            .set(GameState(Some(Move::Moving)))
            .expect("Could not switch states");
    }
}
