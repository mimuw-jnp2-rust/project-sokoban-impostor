use bevy::prelude::*;

use crate::{
    game::game_objects::*,
    resources::{Board, MovementData},
};

use super::{events::MoveEvent, MovableInQuery};

pub fn handle_move(
    mut reader: EventReader<MoveEvent>,
    mut board: ResMut<Board>,
    mut movement_data: ResMut<MovementData>,
    mut query: Query<&mut Position, MovableInQuery>,
) {
    for event in reader.iter() {
        let dir = event.direction;
        let positions = &event.positions;
        for position in positions.iter() {
            let entity = board.get_entity(*position);

            let mut position_component = query
                .get_mut(entity)
                .expect("Moved entity not found in board");
            *position_component = position.neighbour(dir);

            board.move_object(*position, dir);
            movement_data.moved_positions.push(*position);
        }
        movement_data.direction = Some(dir);
    }
}
