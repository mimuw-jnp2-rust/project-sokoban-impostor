use bevy::prelude::*;

use crate::game::game_objects::{Floor, GameObject};
use crate::resources::{AnimationTimer, Board, MovementData};

// checks which entities should move if they are on ice
pub fn handle_ice(
    mut movement_data: ResMut<MovementData>,
    timer: ResMut<AnimationTimer>,
    board: Res<Board>,
) {
    if !timer.0.finished() {
        return;
    }
    let mut positions_on_ice = Vec::new();
    let direction = movement_data
        .direction
        .expect("No direction after animation");
    for position in movement_data.moved_positions.iter() {
        let position = *position;
        if board.get_floor_type(position) != Floor::Ice {
            break; //break in this loop means that this object and all that come before it stop movement
        }
        let object_position = board.get_next_position_for_move(position, direction);
        let object = board.get_object_type(object_position);
        match object {
            GameObject::Empty => {
                positions_on_ice.push(position);
            }
            GameObject::Box => {
                if movement_data
                    .moved_positions
                    .contains(&position.next_position(direction))
                {
                    //found box is already moving
                    positions_on_ice.push(position);
                } else if board.get_floor_type(position.next_position(direction)) == Floor::Ice {
                    // if there are multiple stationary boxes ahead, either the last one moves
                    // (if it's on ice) or they remain stationary otherwise
                    let mut last_box_position = object_position;
                    let mut next_object_position = board.get_next_position_for_move(last_box_position, direction);
                    let mut next_object = board.get_object_type(next_object_position);
                    while next_object == GameObject::Box
                        && board.get_floor_type(next_object_position) == Floor::Ice
                    {
                        last_box_position = next_object_position;
                        next_object_position = board.get_next_position_for_move(next_object_position, direction);
                        next_object = board.get_object_type(next_object_position);
                    }
                    if next_object == GameObject::Empty {
                        positions_on_ice.push(last_box_position);
                    }
                    break;
                    //either way the entity that encountered a stationary entity in front of it must stop, and so do entities before it
                } else {
                    break;
                }
            }
            _ => break,
        }
    }
    movement_data.positions_on_ice = Some(positions_on_ice);
}
