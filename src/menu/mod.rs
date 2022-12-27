use bevy::prelude::*;

mod level_select;
mod main_menu;
mod spawn_button;
use crate::state::GameState;

use level_select::{delete_level_select, handle_level_click, setup_level_select};
use main_menu::{delete_main_menu, handle_menu_click, setup_main_menu};

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_main_menu))
            .add_system_set(SystemSet::on_resume(GameState::MainMenu).with_system(setup_main_menu))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu).with_system(handle_menu_click),
            )
            .add_system_set(SystemSet::on_pause(GameState::MainMenu).with_system(delete_main_menu))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(delete_main_menu));

        app.add_system_set(
            SystemSet::on_enter(GameState::LevelSelect).with_system(setup_level_select),
        )
        .add_system_set(
            SystemSet::on_resume(GameState::LevelSelect).with_system(setup_level_select),
        )
        .add_system_set(
            SystemSet::on_update(GameState::LevelSelect).with_system(handle_level_click),
        )
        .add_system_set(SystemSet::on_exit(GameState::LevelSelect).with_system(delete_level_select))
        .add_system_set(
            SystemSet::on_pause(GameState::LevelSelect).with_system(delete_level_select),
        );
    }
}
