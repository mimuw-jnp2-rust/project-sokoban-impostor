use crate::{state::GameState, consts::TILE_SIZE, resources::{MovementData, InputTimer}};
use bevy::prelude::*;

use crate::game::game_objects::{*, Direction};

fn modify_transform(
    mut transform: Mut<Transform>,
    direction: Direction,
    timer: &ResMut<InputTimer>,
    starting_position: Position,
) {
    match direction {
        Direction::Down => {
            transform.translation.y = (starting_position.y as f32 - timer.0.percent()) * TILE_SIZE;
        }
        Direction::Up => {
            transform.translation.y = (starting_position.y as f32 + timer.0.percent()) * TILE_SIZE;
        }
        Direction::Left => {
            transform.translation.x = (starting_position.x as f32 - timer.0.percent()) * TILE_SIZE;
        }
        Direction::Right => {
            transform.translation.x = (starting_position.x as f32 + timer.0.percent()) * TILE_SIZE;
        }
        _ => (),
    }
}

pub fn move_animation(
    time: Res<Time>,
    movement_data: Res<MovementData>,
    mut query: Query<&mut Transform, Or<(With<Player>, With<Box>)>>,
    mut timer: ResMut<InputTimer>,
    mut app_state: ResMut<State<GameState>>,
) {
    timer.0.tick(time.delta());
    let movement_data = movement_data
        .data
        .as_ref()
        .expect("Movement data not initialized when entering movement");
    let (player_entity, player_position) = movement_data.player_data;
    let player_transform = query
        .get_mut(player_entity)
        .expect("Player entity not found");
    let direction = movement_data.direction;
    modify_transform(player_transform, direction, &timer, player_position);
    for (box_entity, box_position) in movement_data.boxes_data.iter() {
        let transform = query
            .get_mut(*box_entity)
            .expect("Moved box entity not found");
        modify_transform(transform, direction, &timer, *box_position);
    }
    if timer.0.finished() {
        app_state
            .push(GameState::Static)
            .expect("Could not correctly finish movement animation");
    }
}

pub fn end_animation(mut movement_data: ResMut<MovementData>, mut timer: ResMut<InputTimer>) {
    movement_data.data = None;
    timer.0.reset();
}
