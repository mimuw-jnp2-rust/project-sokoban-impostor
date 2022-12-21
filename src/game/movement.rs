use crate::Position;
use bevy::{prelude::*, utils::Duration};

use super::game_objects::{Box, Direction, GameObjects, Player};
use crate::consts::TILE_SIZE;
use crate::resources::{Board, InputTimer};

fn move_box(
    boxes: &mut Query<&mut Transform, With<Box>>,
    board: &mut ResMut<Board>,
    current_entity: Entity,
    next_position: Position,
    old_position: Position,
) {
    let mut transform = boxes
        .get_mut(current_entity)
        .expect("Box not found in board");
    [transform.translation.x, transform.translation.y] =
        [next_position.x, next_position.y].map(|el| TILE_SIZE * el as f32);
    board.entities.remove(&old_position);
    board
        .entities
        .insert(next_position, GameObjects::Box(Some(current_entity)));
}

fn update_box(
    board: &mut ResMut<Board>,
    dir: Direction,
    pos: Position,
    boxes: &mut Query<&mut Transform, With<Box>>,
    current_entity: Entity,
) -> bool {
    let next_position = pos.neighbour(dir);
    let next_position_object = board
        .entities
        .get(&next_position)
        .unwrap_or(&GameObjects::Empty);
    match next_position_object {
        GameObjects::Empty => {
            move_box(boxes, board, current_entity, next_position, pos);
            true
        }
        GameObjects::Box(next_entity) => {
            let next_entity = next_entity.expect("Box not rendered yet");
            let can_move = update_box(board, dir, next_position, boxes, next_entity);
            if can_move {
                move_box(boxes, board, current_entity, next_position, pos);
            }
            can_move
        }
        GameObjects::Wall => false,
    }
}

fn update_pos(
    mut sprite_position: Query<(&mut Player, &mut Transform), Without<Box>>,
    direction: Direction,
    mut timer: ResMut<InputTimer>,
    board: &mut ResMut<Board>,
    boxes: &mut Query<&mut Transform, With<Box>>,
) {
    let (mut player, mut transform) = sprite_position.single_mut();
    let new_position = player.position.neighbour(direction);
    if direction == Direction::None {
        ()
    }
    timer.0.tick(Duration::from_secs(0));
    let object_blocking = board
        .entities
        .get(&new_position)
        .unwrap_or(&GameObjects::Empty);

    match object_blocking {
        GameObjects::Box(entity) => {
            let entity = entity.expect("Box not rendered");
            let can_move = update_box(board, direction, new_position, boxes, entity);
            if can_move {
                [transform.translation.x, transform.translation.y] =
                    [new_position.x, new_position.y].map(|el| TILE_SIZE * el as f32);
                player.position = new_position;
            }
        }
        GameObjects::Wall => (),
        GameObjects::Empty => {
            [transform.translation.x, transform.translation.y] =
                [new_position.x, new_position.y].map(|el| TILE_SIZE * el as f32);
            player.position = new_position;
        }
    }
}

fn set_direction(keyboard_input: Res<Input<KeyCode>>) -> Direction {
    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        Direction::Up
    } else if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        Direction::Down
    } else if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        Direction::Left
    } else if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        Direction::Right
    } else {
        Direction::None
    }
}

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    sprite_position: Query<(&mut Player, &mut Transform), Without<Box>>,
    time: Res<Time>,
    mut timer: ResMut<InputTimer>,
    mut board: ResMut<Board>,
    mut boxes: Query<&mut Transform, With<Box>>,
) {
    // move is only possible once every MOVE_DELAY seconds so only when timer is finished
    let dir = set_direction(keyboard_input);
    if timer.0.finished() {
        update_pos(sprite_position, dir, timer, &mut board, &mut boxes);
    } else {
        timer.0.tick(time.delta());
    }
}
