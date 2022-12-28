use crate::state::GameState;
use crate::Position;
use bevy::prelude::*;

use crate::game::game_objects::{*, Direction};
use crate::resources::*;

fn update_box(
    board: &mut ResMut<Board>,
    dir: Direction,
    pos: Position,
    current_entity: Entity,
    boxes_moved: &mut Vec<(Entity, Position)>,
) -> bool {
    let next_position = pos.neighbour(dir);
    let next_position_object = board
        .entities
        .get(&next_position)
        .unwrap_or(&GameObjects::Empty);
    match next_position_object {
        GameObjects::Empty => {
            boxes_moved.push((current_entity, pos));
            let my_box = board.entities.remove(&pos).expect("Box not found in board");
            board.entities.insert(next_position, my_box);
            true
        }
        GameObjects::Box(next_entity) => {
            let next_entity = next_entity.expect("Box not rendered yet");
            let can_move = update_box(board, dir, next_position, next_entity, boxes_moved);
            if can_move {
                boxes_moved.push((current_entity, pos));
                let my_box = board.entities.remove(&pos).expect("Box not found in board");
                board.entities.insert(next_position, my_box);
            }
            can_move
        }
        GameObjects::Wall => false,
        GameObjects::Player(_) => false,
    }
}

fn update_player(board: &mut ResMut<Board>, movement_data: &mut ResMut<MovementData>, direction: Direction, boxes_moved: &Vec<(Entity, Position)>, app_state: &mut ResMut<State<GameState>>) {
    let player_object = board
        .entities
        .get(&board.player_position)
        .expect("Player not found in board");
    match *player_object {
        GameObjects::Player(Some(player_entity)) => {
            movement_data.data = Some(MovementEntities {
                direction,
                player_data: (player_entity, board.player_position),
                boxes_data: boxes_moved.to_vec(),
            });
            app_state
                .push(GameState::Moving)
                .expect("Could not enter moving state");
        }
        _ => panic!("Something other than rendered player on player position"),
    }
    let player_position = board.player_position;
    let player = board
        .entities
        .remove(&player_position)
        .expect("Player not found in board");
    board
        .entities
        .insert(player_position.neighbour(direction), player);
    board.player_position = player_position.neighbour(direction);
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
) {
    let direction = set_direction(keyboard_input);
    let new_position = board.player_position.neighbour(direction);
    if direction == Direction::None {
        ()
    }
    let object_blocking = board
        .entities
        .get(&new_position)
        .unwrap_or(&GameObjects::Empty);

    match object_blocking {
        GameObjects::Box(entity) => {
            let entity = entity.expect("Box not rendered");
            let mut boxes_moved = Vec::new();
            let can_move = update_box(&mut board, direction, new_position, entity, &mut boxes_moved);
            if can_move {
                update_player(&mut board, &mut movement_data, direction, &boxes_moved, &mut app_state);
            }
        }
        GameObjects::Wall => (),
        GameObjects::Player(_) => (),
        GameObjects::Empty => {
            update_player(&mut board, &mut movement_data, direction, &Vec::new(), &mut app_state)
        }
    }
}
