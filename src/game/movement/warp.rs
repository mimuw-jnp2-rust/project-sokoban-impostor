use bevy::prelude::*;

use crate::{
    game::game_objects::{Floor, GameObject},
    resources::{AnimationTimer, Board, MovementData},
    state::CurrentMap,
};

pub fn handle_warp(
    mut current_map: ResMut<State<CurrentMap>>,
    movement_data: Res<MovementData>,
    mut board: ResMut<Board>,
    timer: Res<AnimationTimer>,
    mut commands: Commands,
) {
    if !timer.0.finished() {
        return;
    }
    for position in movement_data.moved_positions.iter() {
        let position = *position;
        let floor = board.get_floor_type(position);
        if let Floor::Warp(map) = floor {
            let object = board.get_object_type(position);
            match object {
                GameObject::Box => {
                    let entity = board.delete_object(position);
                    commands.entity(entity).despawn();
                    let warp_position = board.get_warp_position(map, board.get_current_map());
                    board.insert_object(warp_position, GameObject::Box);
                }
                GameObject::Player => {
                    let entity = board.delete_object(position);
                    commands.entity(entity).despawn();
                    let warp_position = board.get_warp_position(map, board.get_current_map());
                    board.insert_object(warp_position, GameObject::Player);
                    current_map
                        .push(CurrentMap(Some(map)))
                        .expect("Could not switch maps state");
                    board.set_current_map(map);
                }
                _ => (),
            }
        }
    }
}
