use crate::state::GameState;
use bevy::prelude::*;
use display::{setup_background, setup_move};
use maps::load_starting_map;
use movement::keyboard_input_system;

use self::exit::{exit_to_main_menu, handle_esc};

mod display;
mod exit;
pub mod game_objects;
mod maps;
mod movement;

#[derive(Component)]
pub struct GameItem;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(
                    load_starting_map
                        .before(setup_move)
                        .before(setup_background),
                )
                .with_system(setup_move)
                .with_system(setup_background),
        )
        .add_system_set(
            SystemSet::on_resume(GameState::Game)
                .with_system(
                    load_starting_map
                        .before(setup_move)
                        .before(setup_background),
                )
                .with_system(setup_move)
                .with_system(setup_background),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(keyboard_input_system)
                .with_system(handle_esc),
        );

        app.add_system_set(SystemSet::on_exit(GameState::Game).with_system(exit_to_main_menu))
            .add_system_set(SystemSet::on_pause(GameState::Game).with_system(exit_to_main_menu));
    }
}
