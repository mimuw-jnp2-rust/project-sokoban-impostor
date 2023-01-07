use super::game_objects::{Floor, GameObject, Position};
use crate::{
    consts::INITIAL_MAP,
    resources::{Board, CurrentLevel, MapSize},
};
use bevy::prelude::*;
use std::fs;

pub fn load_starting_map(mut board: ResMut<Board>, current_level: Res<CurrentLevel>) {
    let map_text = fs::read_to_string(current_level.level_map_str).expect("Did not find map file!");
    let mut lines = map_text.lines();
    let maps_amount_str = lines.next().expect("Map file is broken");
    let maps_amount = maps_amount_str
        .parse::<usize>()
        .expect("Maps amount not a number");
    for map in 0..maps_amount {
        board.set_current_map(map);
        let mut split = lines.next().expect("Invalid height provided").split(' ');
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
        let mut y = (height as i32 - 1) / 2;

        for _ in 0..height {
            let line = lines.next().expect("Map provided invalid height");
            for character in line.chars() {
                if x >= -(width as i32 / 2) + width as i32 {
                    panic!("Map provided invalid width");
                }
                if y <= (height as i32 - 1) / 2 - height as i32 {
                    panic!("Map provided invalid height");
                }
                match character {
                    'b' => {
                        board.insert_object(Position { x, y }, GameObject::Box);
                    }
                    'w' => {
                        board.insert_object(Position { x, y }, GameObject::Wall);
                    }
                    'p' => {
                        board.insert_object(Position { x, y }, GameObject::Player);
                    }
                    'g' => {
                        board.insert_floor(Position { x, y }, Floor::Goal);
                    }
                    'i' => {
                        board.insert_floor(Position { x, y }, Floor::Ice);
                    }
                    char if char.is_ascii_digit() => {
                        board.insert_floor(Position { x, y }, Floor::Warp(char.to_digit(10).unwrap() as usize));
                    }
                    _ => (),
                }
                x += 1;
            }
            y -= 1;
            x = -(width as i32 / 2);
        }
        board.set_map_size(MapSize { height, width });
    }
    board.set_current_map(INITIAL_MAP);
}
