use bevy::prelude::*;

use crate::game::game_objects::{Direction, *};
use crate::resources::*;
use crate::state::GameState;

use super::events::MoveEvent;

pub fn handle_keypress(
    keyboard_input: Res<Input<KeyCode>>,
    board: Res<Board>,
    mut writer: EventWriter<MoveEvent>,
    mut app_state: ResMut<State<GameState>>,
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
    let mut next_position = position.next_position(direction);
    positions.push(position);
    //we iterate to see if there is an empty space after some boxes
    while board.get_object_type(next_position) == GameObject::Box {
        position = next_position;
        positions.push(position);
        next_position = next_position.next_position(direction);
    }
    positions.reverse(); //we want to move the last box as first, so that they don't overlap
    let object_blocking = board.get_object_type(next_position);
    if object_blocking == GameObject::Empty {
        writer.send(MoveEvent {
            direction,
            positions,
        });
        app_state
            .set(GameState::Moving)
            .expect("Could not switch states");
    }
}
