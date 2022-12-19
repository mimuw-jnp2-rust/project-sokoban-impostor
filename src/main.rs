use bevy::{prelude::*, utils::HashMap};
mod consts;
mod display;
mod game_objects;
mod movement;
mod resources;
use crate::consts::{BOX_HEIGHT, BOX_WIDTH, MOVE_DELAY};
use crate::display::{setup_background, setup_move};
use crate::game_objects::{GameObjects, Position};
use crate::movement::keyboard_input_system;
use crate::resources::{Board, InputTimer};
fn main() {
    let mut entities = HashMap::new();
    for x in -((BOX_WIDTH / 2) as i32)..((BOX_WIDTH / 2) as i32) {
        entities.insert(
            Position {
                x,
                y: (BOX_HEIGHT / 2 - 1) as i32,
            },
            GameObjects::Wall,
        );
        entities.insert(
            Position {
                x,
                y: -((BOX_HEIGHT / 2) as i32),
            },
            GameObjects::Wall,
        );
    }

    entities.insert(Position{x: 4, y: 6}, GameObjects::Wall);
    for y in -((BOX_HEIGHT / 2 - 1) as i32)..((BOX_HEIGHT / 2 - 1) as i32) {
        entities.insert(
            Position {
                x: (BOX_WIDTH / 2 - 1) as i32,
                y,
            },
            GameObjects::Wall,
        );
        entities.insert(
            Position {
                x: -((BOX_WIDTH / 2) as i32),
                y,
            },
            GameObjects::Wall,
        );
    }

    for x in -2..4 {
        entities.insert(Position { x, y: 1 }, GameObjects::Box(None));
        entities.insert(Position { x, y: -1 }, GameObjects::Box(None));
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
