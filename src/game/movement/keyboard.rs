use bevy::prelude::*;

use crate::game::game_objects::{Direction, *};
use crate::resources::*;

use super::events::MoveEvent;

pub fn handle_keypress(
    keyboard_input: Res<Input<KeyCode>>,
    board: Res<Board>,
    mut writer: EventWriter<MoveEvent>,
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
    let mut next_position = position.neighbour(direction);
    positions.push(position);
    //we iterate to see if there is an empty space after some boxes
    while board.get_object_type(next_position) == GameObjects::Box {
        position = next_position;
        positions.push(position);
        next_position = next_position.neighbour(direction);
    }
    positions.reverse(); //we want to move the last box as first, so that they don't overlap
    if board.get_object_type(next_position) == GameObjects::Empty {
        for position in positions {
            writer.send(MoveEvent {
                direction,
                position,
            });
        }
    }
}
