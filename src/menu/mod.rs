use bevy::prelude::*;

mod level_select;
mod main_menu;
mod spawn_button;
use crate::state::DisplayState;

use level_select::{delete_level_select, handle_level_click, setup_level_select};
use main_menu::{delete_main_menu, handle_menu_click, setup_main_menu};

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(DisplayState::MainMenu).with_system(setup_main_menu),
        )
        .add_system_set(SystemSet::on_resume(DisplayState::MainMenu).with_system(setup_main_menu))
        .add_system_set(SystemSet::on_update(DisplayState::MainMenu).with_system(handle_menu_click))
        .add_system_set(SystemSet::on_pause(DisplayState::MainMenu).with_system(delete_main_menu))
        .add_system_set(SystemSet::on_exit(DisplayState::MainMenu).with_system(delete_main_menu));

        app.add_system_set(
            SystemSet::on_enter(DisplayState::LevelSelect).with_system(setup_level_select),
        )
        .add_system_set(
            SystemSet::on_resume(DisplayState::LevelSelect).with_system(setup_level_select),
        )
        .add_system_set(
            SystemSet::on_update(DisplayState::LevelSelect).with_system(handle_level_click),
        )
        .add_system_set(
            SystemSet::on_exit(DisplayState::LevelSelect).with_system(delete_level_select),
        )
        .add_system_set(
            SystemSet::on_pause(DisplayState::LevelSelect).with_system(delete_level_select),
        );
    }
}
