use bevy::prelude::*;

use crate::{
    resources::{Board, MovementData},
    state::GameState,
    game::game_objects::*
};

use super::{events::MoveEvent, MovableInQuery};

pub fn handle_move(
    mut reader: EventReader<MoveEvent>,
    mut board: ResMut<Board>,
    mut movement_data: ResMut<MovementData>,
    mut app_state: ResMut<State<GameState>>,
    mut query: Query<&mut Position, MovableInQuery>,
) {
    let mut was_movement = false;
    for event in reader.iter() {
        was_movement = true;
        let position = event.position;
        let dir = event.direction;
        let entity = board.get_entity(position);
        let mut position_component = query.get_mut(entity).expect("Moved entity not found in board");
        *position_component = position.neighbour(dir);
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
