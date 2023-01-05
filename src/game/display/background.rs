use bevy::prelude::*;

use crate::consts::*;
use crate::game::game_objects::*;
use crate::resources::{Board, Goals, Images, MapSize};

use super::spawn_entity;

fn offset_coordinate(coord: i32, max: i32) -> i32 {
    coord - (max / 2)
}

//render the entire map based on Board
pub fn setup_background(
    mut commands: Commands,
    mut board: ResMut<Board>,
    map_size: Res<MapSize>,
    goals: Res<Goals>,
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
                GameObjects::Box => {
                    let entity = spawn_entity(
                        Box,
                        &mut commands,
                        images.box_image.clone(),
                        Position { x, y },
                        BOX_Z_INDEX,
                    );
                    board.insert_entity(Position { x, y }, entity);
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
                GameObjects::Player => {
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
        board.insert_object(Position { x: left_border, y }, GameObjects::Wall);
        board.insert_object(Position { x: right_border, y }, GameObjects::Wall);
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
        board.insert_object(Position { x, y: top_border }, GameObjects::Wall);
        board.insert_object(
            Position {
                x,
                y: bottom_border,
            },
            GameObjects::Wall,
        );
    }
}
