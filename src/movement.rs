use bevy::{prelude::*, utils::Duration};
use crate::Position;

use crate::{
    consts::TILE_SIZE,
    game_objects::{Direction, GameObjects, Player},
    resources::{Board, InputTimer},
};

fn update_pos(
    mut sprite_position: Query<(&mut Player, &mut Transform)>,
    direction: Direction,
    mut timer: ResMut<InputTimer>,
    mut board: Res<Board>,
) {
    let (mut player, mut transform) = sprite_position.single_mut();
    let mut new_position = player.position;
    match direction {
        Direction::Up => {
            new_position.y += 1;
            timer.0.tick(Duration::from_secs(0));
        }
        Direction::Down => {
            new_position.y -= 1;
            timer.0.tick(Duration::from_secs(0));
        }
        Direction::Left => {
            new_position.x -= 1;
            timer.0.tick(Duration::from_secs(0));
        }
        Direction::Right => {
            new_position.x += 1;
            timer.0.tick(Duration::from_secs(0));
        }
        Direction::None => (),
    }

    let object_blocking = board
        .entities
        .get(&new_position)
        .unwrap_or(&GameObjects::Empty);
    
    let possibly_box_position = Position{x:  player.position.x, y:  player.position.y, movable: true};

    let maybe_box_object = board
        .entities
        .get(&possibly_box_position)
        .unwrap_or(&GameObjects::Empty);

    if *object_blocking == GameObjects::Empty {
        player.position = new_position;
        [transform.translation.x, transform.translation.y] =
            [player.position.x, player.position.y].map(|el| TILE_SIZE * el as f32);
    }

    if *maybe_box_object == GameObjects::Box {
        println!("BOX");
        board.entities[&possibly_box_position] = GameObjects::Empty;
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
    sprite_position: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
    mut timer: ResMut<InputTimer>,
    mut board: Res<Board>,
) {
    // move is only possible once every MOVE_DELAY seconds so only when timer is finished
    if timer.0.finished() {
        let dir = set_direction(keyboard_input);
        update_pos(sprite_position, dir, timer, board);
    } else {
        timer.0.tick(time.delta());
    }
}
