use bevy::{prelude::*, utils::HashMap};
use consts::{MAP_NAMES, MOVE_ANIMATION_TIME};
use game::display::DisplayPlugin;
use game::GamePlugin;
use game::display::movement::MovementPlugin;
use menu::MenusPlugin;
use resources::{CurrentLevel, Goals, MapSize, MovementData};
use state::GameState;
mod consts;
mod game;
mod menu;
mod resources;
mod state;
use crate::game::game_objects::Position;
use crate::resources::{Board, InputTimer};
use crate::state::DisplayState;

fn main() {
    App::new()
        .insert_resource(InputTimer(Timer::from_seconds(
            MOVE_ANIMATION_TIME,
            TimerMode::Once,
        )))
        .insert_resource(Board {
            entities: HashMap::new(),
            player_position: Position { x: 0, y: 0 },
        })
        .insert_resource(MapSize {
            width: 0,
            height: 0,
        })
        .insert_resource(CurrentLevel {
            level_number: 1,
            level_map_str: MAP_NAMES[0],
        })
        .insert_resource(MovementData {data: None})
        .insert_resource(Goals { goals: Vec::new() })
        .add_plugins(DefaultPlugins)
        .add_plugin(MenusPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(DisplayPlugin)
        .add_plugin(MovementPlugin)
        .add_state(DisplayState::MainMenu)
        .add_state(GameState::NotInGame)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
