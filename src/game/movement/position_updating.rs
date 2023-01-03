use bevy::prelude::*;

use crate::{game::{game_objects::Position, GameItem}, resources::Board};

use super::events::MoveEvent;

pub fn update_board_based_on_position (mut reader: EventReader<MoveEvent>, mut board: ResMut<Board>) {
    for event in reader.iter() {
        let entity = event.entity;
        let dir = event.dir;
        let position = event.position;
        board.move_object(position, dir);
    }
}

