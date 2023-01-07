use bevy::prelude::*;

use crate::{resources::Board, state::CurrentMap, game::game_objects::Floor};

pub fn handle_warp(
    mut current_map: ResMut<State<CurrentMap>>,
    mut board: ResMut<Board>,
) {
    let floor = board.get_floor_type(board.get_player_position());
    if let Floor::Warp(map) = floor {
        println!("{:?}", current_map.current());
        current_map
            .push(CurrentMap(Some(map)))
            .expect("Could not switch maps state");
        board.set_current_map(map);
    }
}
