use bevy::prelude::*;
use consts::*;
use game::display::DisplayPlugin;
use game::movement::MovementPlugin;
use game::GamePlugin;
use menu::MenusPlugin;
use resources::*;
use state::{CurrentMap, DisplayState, GameState};
mod consts;
mod game;
mod labels;
mod menu;
mod resources;
mod state;

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
        .insert_resource(Board::new())
        .insert_resource(CurrentLevel {
            level_number: 1,
            level_map_str: MAP_NAMES[0],
        })
        .insert_resource(MovementData {
            moved_positions: Vec::new(),
            direction: None,
            positions_on_ice: None,
        })
        .insert_resource(BoardStates {
            boards: Vec::new(),
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MenusPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(DisplayPlugin)
        .add_plugin(MovementPlugin)
        .add_state(DisplayState::MainMenu)
        .add_state(GameState(None))
        .add_state(CurrentMap(None))
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
