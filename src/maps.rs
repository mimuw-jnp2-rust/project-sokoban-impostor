use crate::game_objects::Position;
use crate::resources::{Board, MapSize, StartingPosition};
use bevy::prelude::*;
use std::fs;

use crate::consts::MAP_1;

pub fn load_starting_map(
    mut board: ResMut<Board>,
    mut starting_position: ResMut<StartingPosition>,
    mut map_size: ResMut<MapSize>,
) {
    let map_text = fs::read_to_string(MAP_1).expect("Did not find map file!");
    let mut lines = map_text.lines();
    let mut split = lines.next().expect("Map file is broken").split(" ");

    let height = split
        .next()
        .expect("Height not provided")
        .parse::<u32>()
        .expect("Height not a number");

    let width = split
        .next()
        .expect("Width not provided")
        .parse::<u32>()
        .expect("Width not a number");

    let mut x = -(width as i32 / 2);
    let mut y = height as i32 / 2;

    for line in lines {
        for character in line.chars() {
            if x >= -(width as i32 / 2) + width as i32 {
                panic!("Map provided invalid width");
            }
            if y <= height as i32 / 2 - height as i32 {
                panic!("Map provided invalid height");
            }
            match character {
                'b' => {
                    board.entities.insert(
                        Position { x, y },
                        crate::game_objects::GameObjects::Box(None),
                    );
                }
                'w' => {
                    board
                        .entities
                        .insert(Position { x, y }, crate::game_objects::GameObjects::Wall);
                }
                'p' => {
                    *starting_position = StartingPosition {
                        position: Position { x, y },
                    }
                }
                _ => (),
            }
            x += 1;
        }
        y -= 1;
        x = -(width as i32 / 2);
    }
    *map_size = MapSize { height, width };
}
