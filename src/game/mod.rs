use crate::state::{DisplayState, GameState};
use bevy::prelude::*;
use display::{background::setup_background, render::spawn_player};
use exit::{exit_to_main_menu, handle_esc};
use maps::load_starting_map;
use victory::{delete_win, handle_win, handle_win_click, setup_win};

pub mod display;
mod exit;
pub mod game_objects;
mod maps;
mod victory;
pub mod movement;

#[derive(Component)]
pub struct GameItem;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(DisplayState::Game)
                .with_system(
                    load_starting_map
                        .before(spawn_player)
                        .before(setup_background),
                )
                .with_system(set_game_state),
        )
        .add_system_set(
            SystemSet::on_resume(DisplayState::Game)
                .with_system(
                    load_starting_map
                        .before(spawn_player)
                        .before(setup_background),
                )
                .with_system(set_game_state),
        )
        .add_system_set(
            SystemSet::on_update(DisplayState::Game)
                .with_system(handle_esc)
                .with_system(handle_win),
        );

        app.add_system_set(
            SystemSet::on_exit(DisplayState::Game)
                .with_system(exit_to_main_menu)
                .with_system(reset_game_state),
        )
        .add_system_set(
            SystemSet::on_pause(DisplayState::Game)
                .with_system(exit_to_main_menu)
                .with_system(reset_game_state),
        );

        app.add_system_set(SystemSet::on_enter(DisplayState::Victory).with_system(setup_win))
            .add_system_set(SystemSet::on_resume(DisplayState::Victory).with_system(setup_win))
            .add_system_set(
                SystemSet::on_update(DisplayState::Victory).with_system(handle_win_click),
            )
            .add_system_set(SystemSet::on_pause(DisplayState::Victory).with_system(delete_win))
            .add_system_set(SystemSet::on_exit(DisplayState::Victory).with_system(delete_win));
    }
}

fn set_game_state(mut game_state: ResMut<State<GameState>>) {
    game_state
        .push(GameState::Static)
        .expect("Could not set static game state");
}

fn reset_game_state(mut game_state: ResMut<State<GameState>>) {
    game_state
        .push(GameState::NotInGame)
        .expect("Could not reset game state");
}
