use bevy::prelude::*;

use crate::game::{resources::Board, game_objects::{HiddenWall, Floor}};

use super::events::{EnteredFloorEvent, ExitedFloorEvent};

pub fn handle_button(
    mut entered_reader: EventReader<EnteredFloorEvent>,
    mut board: ResMut<Board>,
) {
    for event in entered_reader.iter() {
        if event.floor == Floor::Button {
            board.toggle_hiding_wall();
        }
    }
    
}
