use bevy::prelude::*;
use consts::*;
use game::display::DisplayPlugin;
use game::movement::MovementPlugin;
use game::GamePlugin;
use menu::MenusPlugin;
use resources::*;
use state::{CurrentMap, DisplayState, GameState};
mod consts;
mod exit;
mod game;
mod labels;
mod menu;
mod resources;
mod state;
mod utils;

fn main() {
    App::new()
        .insert_resource(CurrentLevel {
            level_number: 1,
            level_map_str: MAP_NAMES[0],
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
