use crate::{
    consts::TILE_SIZE,
    resources::{Board, InputTimer, MovementData},
    state::GameState,
};
use bevy::prelude::*;

use crate::game::game_objects::{Direction, *};

use super::{
    consts::{INTERVAL_DISTANCE_1, SPEED_1, TIME_INTERVAL_1},
    events::MoveEvent,
    MovableInQuery,
};

fn animation_weight(number: f32) -> f32 {
    //this is so that the movement isn't uniform; first move with SPEED_1, and then with speed such that we move an entire tile
    if number < TIME_INTERVAL_1 {
        number * SPEED_1
    } else {
        (number - TIME_INTERVAL_1) * ((1. - INTERVAL_DISTANCE_1) / (1. - TIME_INTERVAL_1))
            + INTERVAL_DISTANCE_1
    }
}

fn modify_transform(
    mut transform: Mut<Transform>,
    direction: Direction,
    timer: &ResMut<InputTimer>,
    starting_position: Position,
) {
    match direction {
        Direction::Down => {
            transform.translation.y =
                (starting_position.y as f32 - animation_weight(timer.0.percent())) * TILE_SIZE;
        }
        Direction::Up => {
            transform.translation.y =
                (starting_position.y as f32 + animation_weight(timer.0.percent())) * TILE_SIZE;
        }
        Direction::Left => {
            transform.translation.x =
                (starting_position.x as f32 - animation_weight(timer.0.percent())) * TILE_SIZE;
        }
        Direction::Right => {
            transform.translation.x =
                (starting_position.x as f32 + animation_weight(timer.0.percent())) * TILE_SIZE;
        }
    }
}

pub fn move_animation(
    time: Res<Time>,
    movement_data: Res<MovementData>,
    mut query: Query<&mut Transform, MovableInQuery>,
    mut timer: ResMut<InputTimer>,
    board: Res<Board>,
) {
    timer.0.tick(time.delta());
    let direction_opt = movement_data.direction;
    if let Some(direction) = direction_opt {
        for position in movement_data.moved_positions.iter() {
            let entity = board.get_entity(*position);
            let transform = query.get_mut(entity).expect("Moved box entity not found");
            modify_transform(transform, direction, &timer, position.previous_position(direction));
        }
    }
}

pub fn end_animation(mut movement_data: ResMut<MovementData>, mut timer: ResMut<InputTimer>) {
    movement_data.moved_positions.clear();
    movement_data.direction = None;
    timer.0.reset();
}

pub fn handle_ice(
    mut movement_data: ResMut<MovementData>,
    mut timer: ResMut<InputTimer>,
    mut app_state: ResMut<State<GameState>>,
    board: Res<Board>,
    mut writer: EventWriter<MoveEvent>,
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
            break;      //break in this loop means that this object and all that come before it stop movement
        }
        let object = board.get_object_type(position.next_position(direction));
        match object {
            GameObject::Empty => positions_on_ice.push(position),
            GameObject::Box => {
                if movement_data.moved_positions.contains(&position.next_position(direction)) {     //found box is already moving
                    positions_on_ice.push(position);
                }
                else if board.get_floor_type(position.next_position(direction)) == Floor::Ice {
                    // if there are multiple stationary boxes ahead, either the last one moves
                    // (if it's on ice) or they remain stationary otherwise
                    let mut last_box_position = position.next_position(direction);
                    let mut next_object_position = last_box_position.next_position(direction);
                    let mut next_object = board.get_object_type(next_object_position);
                    while next_object == GameObject::Box && board.get_floor_type(next_object_position) == Floor::Ice {
                        last_box_position = next_object_position;
                        next_object_position = next_object_position.next_position(direction);
                        next_object = board.get_object_type(next_object_position);
                    }
                    if next_object == GameObject::Empty {
                        positions_on_ice.push(last_box_position);
                    }
                    break;
                    //either way the entity that encountered a stationary entity in front of it must stop, and so do entities before it
                }
                else {
                    break;
                }
            }
            _ => break,
        }
    }
    if positions_on_ice.is_empty() {
        app_state
            .set(GameState::Static)
            .expect("Could not correctly finish movement animation");
    } else {
        movement_data.direction = None;
        movement_data.moved_positions.clear();
        writer.send(MoveEvent {
            direction,
            positions: positions_on_ice,
        });
        timer.0.reset();
    }
}
