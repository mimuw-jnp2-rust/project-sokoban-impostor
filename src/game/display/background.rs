use bevy::prelude::*;

use crate::consts::*;
use crate::game::game_objects::*;
use crate::resources::{Board, Goals, Images, MapSize};

use super::render::spawn_entity;

fn offset_coordinate(coord: u32, max: u32) -> i32 {
    coord as i32 - (max as i32 / 2)
}

//render the entire map based on Board
pub fn setup_background(
    mut commands: Commands,
    mut board: ResMut<Board>,
    map_size: Res<MapSize>,
    goals: Res<Goals>,
    images: Res<Images>,
) {
    let bottom_border = offset_coordinate(0, map_size.height);
    let top_border = offset_coordinate(map_size.height - 1, map_size.height);
    let left_border = offset_coordinate(0, map_size.width);
    let right_border = offset_coordinate(map_size.width - 1, map_size.width);

    // render all objects found in board
    for y in bottom_border..(top_border + 1) {
        for x in left_border..(right_border + 1) {
            let game_object = board.get_object_type(Position { x, y });
            match game_object {
                GameObjects::Box => {
                    let entity = spawn_entity(
                        Box,
                        &mut commands,
                        images.box_image.clone(),
                        Position { x, y },
                        BOX_Z_INDEX,
                    );
                    board.insert_entity(Position { x, y }, entity)
                }
                GameObjects::Wall => {
                    spawn_entity(
                        Wall,
                        &mut commands,
                        images.wall_image.clone(),
                        Position { x, y },
                        WALL_Z_INDEX,
                    );
                }
                _ => (),
            }
            //spawn background for every tile
            spawn_entity(
                Background,
                &mut commands,
                images.tile_image.clone(),
                Position { x, y },
                TILE_Z_INDEX,
            );
        }
    }
    for position in goals.goals.iter() {
        spawn_entity(
            Goal,
            &mut commands,
            images.goal_image.clone(),
            *position,
            GOAL_Z_INDEX,
        );
    }
}

pub fn setup_border(
    mut commands: Commands,
    mut board: ResMut<Board>,
    map_size: Res<MapSize>,
    images: Res<Images>,
) {
    let bottom_border = offset_coordinate(0, map_size.height);
    let top_border = offset_coordinate(map_size.height - 1, map_size.height);
    let left_border = offset_coordinate(0, map_size.width);
    let right_border = offset_coordinate(map_size.width - 1, map_size.width);
        //spawn horizontal border for the level and insert it to board
    for y in (bottom_border - 1)..(top_border + 2) {
        spawn_entity(
            Wall,
            &mut commands,
            images.wall_image.clone(),
            Position {
                x: left_border - 1,
                y,
            },
            WALL_Z_INDEX,
        );
        spawn_entity(
            Wall,
            &mut commands,
            images.wall_image.clone(),
            Position {
                x: right_border + 1,
                y,
            },
            WALL_Z_INDEX,
        );
        board.insert_object(
            Position {
                x: left_border - 1,
                y,
            },
            GameObjects::Wall,
        );
        board.insert_object(
            Position {
                x: right_border + 1,
                y,
            },
            GameObjects::Wall,
        );
    }
    //spawn vertical borders for the level and insert it to board
    for x in (left_border - 1)..(right_border + 2) {
        spawn_entity(
            Wall,
            &mut commands,
            images.wall_image.clone(),
            Position {
                x,
                y: top_border + 1,
            },
            WALL_Z_INDEX,
        );
        spawn_entity(
            Wall,
            &mut commands,
            images.wall_image.clone(),
            Position {
                x,
                y: bottom_border - 1,
            },
            WALL_Z_INDEX,
        );
        board.insert_object(
            Position {
                x,
                y: top_border + 1,
            },
            GameObjects::Wall,
        );
        board.insert_object(
            Position {
                x,
                y: bottom_border - 1,
            },
            GameObjects::Wall,
        );
    }
}