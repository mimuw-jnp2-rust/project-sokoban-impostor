use bevy::{
    prelude::*,
    utils::HashMap,
};
mod consts;
mod game_objects;
mod movement;
mod resources;
mod display;
use crate::consts::MOVE_DELAY;
use crate::game_objects::{Position, GameObjects};
use crate::movement::keyboard_input_system;
use crate::resources::{Board, InputTimer};
use crate::display::{setup_background, setup_move};
fn main() {
    let mut entities = HashMap::new();
    for x in -2..4 {
        entities.insert(Position { x, y: 3 }, GameObjects::Wall);
        entities.insert(Position { x, y: -3 }, GameObjects::Wall);
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