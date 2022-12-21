use bevy::{prelude::*, utils::HashMap};
use consts::MAP_NAMES;
use game::GamePlugin;
use menu::MenusPlugin;
use resources::{CurrentLevel, Goals, MapSize, StartingPosition};
mod consts;
mod game;
mod menu;
mod resources;
mod state;
use crate::consts::MOVE_DELAY;
use crate::game::game_objects::Position;
use crate::resources::{Board, InputTimer};
use crate::state::GameState;

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
        .insert_resource(CurrentLevel {
            level_number: 1,
            level_map_str: MAP_NAMES[0],
        })
        .insert_resource(Goals { goals: Vec::new() })
        .add_plugins(DefaultPlugins)
        .add_plugin(MenusPlugin)
        .add_plugin(GamePlugin)
        .add_state(GameState::MainMenu)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
