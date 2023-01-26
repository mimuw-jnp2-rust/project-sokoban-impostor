use super::resources::{AnimationTimer, MovementData};
use crate::consts::TILE_SIZE;
use crate::game::resources::Board;
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
    timer: &ResMut<AnimationTimer>,
    starting_position: Position,
    movement_data: &Res<MovementData>,
) {
    let distance = if movement_data.is_on_ice {
        //on ice we want to have uniform movement animation
        timer.0.percent()
    } else {
        animation_weight(timer.0.percent())
    };
    match direction {
        Direction::Down => {
            transform.translation.y = (starting_position.y as f32 - distance) * TILE_SIZE;
        }
        Direction::Up => {
            transform.translation.y = (starting_position.y as f32 + distance) * TILE_SIZE;
        }
        Direction::Left => {
            transform.translation.x = (starting_position.x as f32 - distance) * TILE_SIZE;
        }
        Direction::Right => {
            transform.translation.x = (starting_position.x as f32 + distance) * TILE_SIZE;
        }
    }
}

pub fn move_animation(
    time: Res<Time>,
    movement_data: Res<MovementData>,
    mut query: Query<&mut Transform, MovableInQuery>,
    mut timer: ResMut<AnimationTimer>,
    board: Res<Board>,
) {
    timer.0.tick(time.delta());
    let direction_opt = movement_data.direction;
    if let Some(direction) = direction_opt {
        for position in movement_data.moved_positions.iter() {
            let entity_opt = board.get_entity(*position);
            if let Some(entity) = entity_opt {
                let transform = query.get_mut(entity).expect("Moved box entity not found");
                modify_transform(
                    transform,
                    direction,
                    &timer,
                    position.previous_position(direction),
                    &movement_data,
                );
            }
        }
    }
}

pub fn end_animation(mut movement_data: ResMut<MovementData>, mut timer: ResMut<AnimationTimer>) {
    movement_data.moved_positions.clear();
    movement_data.direction = None;
    timer.0.reset();
}
