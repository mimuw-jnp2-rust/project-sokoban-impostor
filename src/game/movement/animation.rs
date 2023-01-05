use crate::{
    consts::TILE_SIZE,
    resources::{Board, InputTimer, MovementData},
    state::GameState,
};
use bevy::prelude::*;

use crate::game::game_objects::{Direction, *};

use super::{
    consts::{INTERVAL_DISTANCE_1, SPEED_1, TIME_INTERVAL_1},
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
    mut app_state: ResMut<State<GameState>>,
    board: Res<Board>,
) {
    timer.0.tick(time.delta());
    let direction_opt = movement_data.direction;
    if let Some(direction) = direction_opt {
        for position in movement_data.moved_positions.iter() {
            let entity = board.get_entity(position.neighbour(direction));
            let transform = query.get_mut(entity).expect("Moved box entity not found");
            modify_transform(transform, direction, &timer, *position);
        }
    }
    if timer.0.finished() {
        app_state
            .push(GameState::Static)
            .expect("Could not correctly finish movement animation");
    }
}

pub fn end_animation(mut movement_data: ResMut<MovementData>, mut timer: ResMut<InputTimer>) {
    movement_data.moved_positions.clear();
    movement_data.direction = None;
    timer.0.reset();
}
