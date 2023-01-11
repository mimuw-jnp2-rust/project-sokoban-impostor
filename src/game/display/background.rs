use bevy::prelude::*;

use crate::consts::*;
use crate::game::game_objects::*;
use crate::resources::{Board, Images};

use super::spawn_entity;

fn offset_coordinate(coord: i32, max: i32) -> i32 {
    coord - (max / 2)
}

//render the entire map based on Board
pub fn setup_background(mut commands: Commands, mut board: ResMut<Board>, images: Res<Images>) {
    let map_size = board.get_map_size();
    let bottom_border = offset_coordinate(0, map_size.height as i32);
    let top_border = offset_coordinate(map_size.height as i32 - 1, map_size.height as i32);
    let left_border = offset_coordinate(0, map_size.width as i32);
    let right_border = offset_coordinate(map_size.width as i32 - 1, map_size.width as i32);
    let map = board.get_current_map();
    // render all objects found in board
    for y in bottom_border..(top_border + 1) {
        for x in left_border..(right_border + 1) {
            let position = Position { x, y, map };
            let game_object = board.get_object_type(position);
            match game_object {
                GameObject::Box => {
                    let image;
                    if board.get_floor_type(position) == Floor::Goal {
                        image = images.box_on_goal_image.clone();
                    } else {
                        image = images.box_image.clone();
                    }
                    let entity = spawn_entity(Box, &mut commands, image, position, OBJECT_Z_INDEX);
                    board.insert_entity(position, entity);
                }
                GameObject::Wall => {
                    spawn_entity(
                        Wall,
                        &mut commands,
                        images.wall_image.clone(),
                        position,
                        OBJECT_Z_INDEX,
                    );
                }
                GameObject::Player => {
                    let entity = spawn_entity(
                        Player,
                        &mut commands,
                        images.player_image.clone(),
                        position,
                        OBJECT_Z_INDEX,
                    );
                    board.insert_entity(position, entity);
                }
                _ => (),
            }
        }
    }
    for y in bottom_border..(top_border + 1) {
        for x in left_border..(right_border + 1) {
            let position = Position { x, y, map };
            let floor = board.get_floor_type(position);
            match floor {
                Floor::Ice => {
                    spawn_entity(
                        Ice,
                        &mut commands,
                        images.ice_image.clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                }
                Floor::Tile => {
                    spawn_entity(
                        Background,
                        &mut commands,
                        images.tile_image.clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                }
                Floor::Goal => {
                    spawn_entity(
                        Goal,
                        &mut commands,
                        images.goal_image.clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                }
                Floor::Warp(_) => {
                    spawn_entity(
                        Warp,
                        &mut commands,
                        images.warp_image.clone(),
                        position,
                        FLOOR_Z_INDEX,
                    );
                }
            }
        }
    }
}

pub fn setup_border(mut commands: Commands, mut board: ResMut<Board>, images: Res<Images>) {
    let map_size = board.get_map_size();
    let bottom_border = offset_coordinate(-1, map_size.height as i32);
    let top_border = offset_coordinate(map_size.height as i32, map_size.height as i32);
    let left_border = offset_coordinate(-1, map_size.width as i32);
    let right_border = offset_coordinate(map_size.width as i32, map_size.width as i32);
    let map = board.get_current_map();
    //spawn horizontal border for the level and insert it to board
    for y in bottom_border..(top_border + 1) {
        spawn_entity(
            Wall,
            &mut commands,
            images.wall_image.clone(),
            Position {
                x: left_border,
                y,
                map,
            },
            OBJECT_Z_INDEX,
        );
        spawn_entity(
            Wall,
            &mut commands,
            images.wall_image.clone(),
            Position {
                x: right_border,
                y,
                map,
            },
            OBJECT_Z_INDEX,
        );
        board.insert_object(
            Position {
                x: left_border,
                y,
                map,
            },
            GameObject::Wall,
        );
        board.insert_object(
            Position {
                x: right_border,
                y,
                map,
            },
            GameObject::Wall,
        );
    }
    //spawn vertical borders for the level and insert it to board
    for x in left_border..(right_border + 1) {
        spawn_entity(
            Wall,
            &mut commands,
            images.wall_image.clone(),
            Position {
                x,
                y: top_border,
                map,
            },
            OBJECT_Z_INDEX,
        );
        spawn_entity(
            Wall,
            &mut commands,
            images.wall_image.clone(),
            Position {
                x,
                y: bottom_border,
                map,
            },
            OBJECT_Z_INDEX,
        );
        board.insert_object(
            Position {
                x,
                y: top_border,
                map,
            },
            GameObject::Wall,
        );
        board.insert_object(
            Position {
                x,
                y: bottom_border,
                map,
            },
            GameObject::Wall,
        );
    }
}
