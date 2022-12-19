use bevy::{prelude::*, utils::HashMap};
use maps::load_starting_map;
use resources::{MapSize, StartingPosition};
mod consts;
mod display;
mod game_objects;
mod maps;
mod movement;
mod resources;
use crate::consts::MOVE_DELAY;
use crate::display::{setup_background, setup_move};
use crate::game_objects::Position;
use crate::movement::keyboard_input_system;
use crate::resources::{Board, InputTimer};
fn main() {
    App::new()
        .insert_resource(InputTimer(Timer::from_seconds(
            MOVE_DELAY,
            TimerMode::Repeating,
        )))
        .insert_resource(Board {
            entities: HashMap::new(),
        })
        .insert_resource(StartingPosition {
            position: Position { x: 0, y: 0 },
        })
        .insert_resource(MapSize {
            width: 0,
            height: 0,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(
            load_starting_map
                .before(setup_background)
                .before(setup_move),
        )
        .add_startup_system(setup_background)
        .add_startup_system(setup_move)
        .add_system(keyboard_input_system)
        .run();
}
