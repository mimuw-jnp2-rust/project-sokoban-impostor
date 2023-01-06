use bevy::prelude::*;
use consts::{MAP_NAMES, MOVE_ANIMATION_TIME, RESTART_TIME};
use game::display::DisplayPlugin;
use game::movement::MovementPlugin;
use game::GamePlugin;
use menu::MenusPlugin;
use resources::{CurrentLevel, MapSize, MovementData, VictoryTimer, RestartTimer};
use state::GameState;
mod consts;
mod game;
mod labels;
mod menu;
mod resources;
mod state;
use crate::resources::{Board, InputTimer};
use crate::state::DisplayState;

fn main() {
    App::new()
        .insert_resource(InputTimer(Timer::from_seconds(
            MOVE_ANIMATION_TIME,
            TimerMode::Once,
        )))
        .insert_resource(VictoryTimer(Timer::from_seconds(
            MOVE_ANIMATION_TIME * 2.,
            TimerMode::Once,
        )))
        .insert_resource(RestartTimer(Timer::from_seconds(RESTART_TIME, TimerMode::Once)))
        .insert_resource(Board::new())
        .insert_resource(MapSize {
            width: 0,
            height: 0,
        })
        .insert_resource(CurrentLevel {
            level_number: 1,
            level_map_str: MAP_NAMES[0],
        })
        .insert_resource(MovementData {
            moved_positions: Vec::new(),
            direction: None,
        })
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
