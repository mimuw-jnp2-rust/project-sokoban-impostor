use bevy::prelude::*;

use crate::game::resources::Board;

use super::events::MoveEvent;
use super::resources::MovementData;

pub fn handle_move(
    mut reader: EventReader<MoveEvent>,
    mut board: ResMut<Board>,
    mut movement_data: ResMut<MovementData>,
) {
    //updates the positions in MoveEvent without checking anything. We assume that the positions are already correct
    for event in reader.iter() {
        let dir = event.direction;
        let positions = &event.positions;
        for position in positions.iter() {
            board.move_object(*position, dir);
            movement_data
                .moved_positions
                .push(position.next_position(dir));
        }
        movement_data.direction = Some(dir);
    }
}
