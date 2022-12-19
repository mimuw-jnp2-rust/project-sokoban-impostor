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
    let mut height = 0;
    let mut width = 0;
    let mut x = 0;
    let mut y = 0;
    for line in map_text.lines() {
        if height == 0 && width == 0 {
            let mut split = line.split(" ");
            height = split
                .next()
                .expect("Height not provided")
                .parse::<u32>()
                .expect("Height not a number");
            width = split
                .next()
                .expect("Width not provided")
                .parse::<u32>()
                .expect("Width not a number");
            x = -(width as i32 / 2);
            y = -(height as i32 / 2);
        } else {
            for character in line.chars() {
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
            y += 1;
            x = -(width as i32 / 2);
        }
    }
    *map_size = MapSize { height, width };
}
