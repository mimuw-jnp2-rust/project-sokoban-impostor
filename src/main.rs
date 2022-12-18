use bevy::{
    prelude::*,
    utils::HashMap,
};
mod consts;
mod game_objects;
mod movement;
mod resources;
mod display;
use crate::consts::{MOVE_DELAY, BOX_HEIGHT, BOX_WIDTH};
use crate::game_objects::{Position, GameObjects};
use crate::movement::keyboard_input_system;
use crate::resources::{Board, InputTimer};
use crate::display::{setup_background, setup_move};
fn main() {
    let mut entities = HashMap::new();
    for x in -((BOX_WIDTH/2) as i32)..((BOX_WIDTH/2) as i32) {
        entities.insert(Position { movable: false, x, y: (BOX_HEIGHT/2 - 1) as i32 }, GameObjects::Wall);
        entities.insert(Position { movable: false, x, y: -((BOX_HEIGHT/2) as i32) }, GameObjects::Wall);
    }

    for y in -((BOX_HEIGHT/2 - 1) as i32)..((BOX_HEIGHT/2 - 1) as i32) {
        entities.insert(Position { movable: false, x: (BOX_WIDTH/2 - 1) as i32 , y}, GameObjects::Wall);
        entities.insert(Position { movable: false, x: -((BOX_WIDTH/2) as i32) , y}, GameObjects::Wall);
    }

    for x in -2..4 {
        entities.insert(Position { movable: true, x, y: 1 }, GameObjects::Box);
        entities.insert(Position { movable: true, x, y: -1 }, GameObjects::Box);
    }

    App::new()
        .insert_resource(InputTimer(Timer::from_seconds(
            MOVE_DELAY,
            TimerMode::Repeating,
        )))
        .insert_resource(Board { entities })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_background)
        .add_startup_system(setup_move)
        .add_system(keyboard_input_system)
        .run();
}