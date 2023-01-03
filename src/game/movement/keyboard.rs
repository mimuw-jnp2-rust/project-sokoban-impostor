use crate::state::GameState;
use crate::Position;
use bevy::prelude::*;

use crate::game::game_objects::{Direction, *};
use crate::resources::*;

fn update_box(
    board: &mut ResMut<Board>,
    query: &mut Query<&mut Position, With<Box>>,
    dir: Direction,
    pos: Position,
    current_entity: Entity,
    boxes_moved: &mut Vec<(Entity, Position)>,
) -> bool {
    let next_position = pos.neighbour(dir);
    let next_position_object = board.get_object_type(next_position);
    match next_position_object {
        GameObjects::Empty => {
            boxes_moved.push((current_entity, pos));
            board.move_object(pos, dir);
            let mut position = query
                .get_mut(current_entity)
                .expect("Box not found in query");
            *position = position.neighbour(dir);
            true
        }
        GameObjects::Box => {
            let next_entity = board.get_entity(next_position);
            let can_move = update_box(board, query, dir, next_position, next_entity, boxes_moved);
            if can_move {
                boxes_moved.push((current_entity, pos));
                board.move_object(pos, dir);
                let mut position = query
                    .get_mut(current_entity)
                    .expect("Box not found in query");
                *position = position.neighbour(dir);
            }
            can_move
        }
        GameObjects::Wall => false,
        GameObjects::Player => false,
    }
}

fn update_player(
    board: &mut ResMut<Board>,
    movement_data: &mut ResMut<MovementData>,
    direction: Direction,
    boxes_moved: &[(Entity, Position)],
    app_state: &mut ResMut<State<GameState>>,
) {
    let player_position = board.get_player_position();
    let player_entity = board.get_entity(player_position);
    movement_data.data = Some(MovementEntities {
        direction,
        player_data: (player_entity, player_position),
        boxes_data: boxes_moved.to_vec(),
    });
    app_state
        .push(GameState::Moving)
        .expect("Could not enter moving state");
    
    board.move_object(player_position, direction);
}

fn set_direction(keyboard_input: Res<Input<KeyCode>>) -> Direction {
    if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
        Direction::Up
    } else if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
        Direction::Down
    } else if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
        Direction::Left
    } else if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        Direction::Right
    } else {
        Direction::None
    }
}

pub fn handle_keypress(
    keyboard_input: Res<Input<KeyCode>>,
    mut board: ResMut<Board>,
    mut movement_data: ResMut<MovementData>,
    mut app_state: ResMut<State<GameState>>,
    mut query: Query<&mut Position, With<Box>>,
) {
    let direction = set_direction(keyboard_input);
    let new_position = board.get_player_position().neighbour(direction);
    if direction == Direction::None {
        ()
    }
    let object_blocking = board.get_object_type(new_position);

    match object_blocking {
        GameObjects::Box => {
            let entity = board.get_entity(new_position);
            let mut boxes_moved = Vec::new();
            let can_move = update_box(
                &mut board,
                &mut query,
                direction,
                new_position,
                entity,
                &mut boxes_moved,
            );
            if can_move {
                update_player(
                    &mut board,
                    &mut movement_data,
                    direction,
                    &boxes_moved,
                    &mut app_state,
                );
            }
        }
        GameObjects::Wall => (),
        GameObjects::Player => (),
        GameObjects::Empty => update_player(
            &mut board,
            &mut movement_data,
            direction,
            &Vec::new(),
            &mut app_state,
        ),
    }
}
