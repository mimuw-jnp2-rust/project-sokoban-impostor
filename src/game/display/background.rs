use bevy::prelude::*;

use crate::consts::*;
use crate::game::game_objects::*;
use crate::resources::{Board, Images, MapSize};

use super::spawn_entity;

fn offset_coordinate(coord: i32, max: i32) -> i32 {
    coord - (max / 2)
}

//render the entire map based on Board
pub fn setup_background(
    mut commands: Commands,
    mut board: ResMut<Board>,
    map_size: Res<MapSize>,
    images: Res<Images>,
) {
    let bottom_border = offset_coordinate(0, map_size.height as i32);
    let top_border = offset_coordinate(map_size.height as i32 - 1, map_size.height as i32);
    let left_border = offset_coordinate(0, map_size.width as i32);
    let right_border = offset_coordinate(map_size.width as i32 - 1, map_size.width as i32);

    // render all objects found in board
    for y in bottom_border..(top_border + 1) {
        for x in left_border..(right_border + 1) {
            let game_object = board.get_object_type(Position { x, y });
            match game_object {
                GameObject::Box => {
                    let entity = spawn_entity(
                        Box,
                        &mut commands,
                        images.box_image.clone(),
                        Position { x, y },
                        BOX_Z_INDEX,
                    );
                    board.insert_entity(Position { x, y }, entity);
                }
                GameObject::Wall => {
                    spawn_entity(
                        Wall,
                        &mut commands,
                        images.wall_image.clone(),
                        Position { x, y },
                        WALL_Z_INDEX,
                    );
                }
                GameObject::Player => {
                    let entity = spawn_entity(
                        Player,
                        &mut commands,
                        images.player_image.clone(),
                        Position { x, y },
                        PLAYER_Z_INDEX,
                    );
                    board.insert_entity(Position { x, y }, entity);
                }
                _ => (),
            }
        }
    }
    for y in bottom_border..(top_border + 1) {
        for x in left_border..(right_border + 1) {
            let position = Position { x, y };
            let floor = board.get_floor_type(position);
            match floor {
                Floor::Ice => {
                    spawn_entity(
                        Ice,
                        &mut commands,
                        images.ice_image.clone(),
                        position,
                        ICE_Z_INDEX,
                    );
                }
                Floor::Tile => {
                    spawn_entity(
                        Background,
                        &mut commands,
                        images.tile_image.clone(),
                        position,
                        TILE_Z_INDEX,
                    );
                }
                Floor::Goal => {
                    spawn_entity(
                        Goal,
                        &mut commands,
                        images.goal_image.clone(),
                        position,
                        GOAL_Z_INDEX,
                    );
                }
            }
        }
    }
    // for position in goals.goals.iter() {
    //     spawn_entity(
    //         Goal,
    //         &mut commands,
    //         images.goal_image.clone(),
    //         *position,
    //         GOAL_Z_INDEX,
    //     );
    // }
}

pub fn setup_border(
    mut commands: Commands,
    mut board: ResMut<Board>,
    map_size: Res<MapSize>,
    images: Res<Images>,
) {
    let bottom_border = offset_coordinate(-1, map_size.height as i32);
    let top_border = offset_coordinate(map_size.height as i32, map_size.height as i32);
    let left_border = offset_coordinate(-1, map_size.width as i32);
    let right_border = offset_coordinate(map_size.width as i32, map_size.width as i32);
    //spawn horizontal border for the level and insert it to board
    for y in bottom_border..(top_border + 1) {
        spawn_entity(
            Wall,
            &mut commands,
            images.wall_image.clone(),
            Position { x: left_border, y },
            WALL_Z_INDEX,
        );
        spawn_entity(
            Wall,
            &mut commands,
            images.wall_image.clone(),
            Position { x: right_border, y },
            WALL_Z_INDEX,
        );
        board.insert_object(Position { x: left_border, y }, GameObject::Wall);
        board.insert_object(Position { x: right_border, y }, GameObject::Wall);
    }
    //spawn vertical borders for the level and insert it to board
    for x in left_border..(right_border + 1) {
        spawn_entity(
            Wall,
            &mut commands,
            images.wall_image.clone(),
            Position { x, y: top_border },
            WALL_Z_INDEX,
        );
        spawn_entity(
            Wall,
            &mut commands,
            images.wall_image.clone(),
            Position {
                x,
                y: bottom_border,
            },
            WALL_Z_INDEX,
        );
        board.insert_object(Position { x, y: top_border }, GameObject::Wall);
        board.insert_object(
            Position {
                x,
                y: bottom_border,
            },
            GameObject::Wall,
        );
    }
}
