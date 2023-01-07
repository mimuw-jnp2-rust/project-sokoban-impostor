use crate::{
    labels::Labels,
    state::{DisplayState, GameState}, resources::RestartTimer,
};
use bevy::prelude::*;
use exit::{exit_to_main_menu, handle_esc};
use maps::load_starting_map;
use victory::{delete_win, handle_win, handle_win_click, setup_win};

use self::victory::handle_box_highlight;

pub mod display;
mod exit;
pub mod game_objects;
mod maps;
pub mod movement;
mod victory;

#[derive(Component)]
pub struct GameItem;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(DisplayState::Game)
                .with_system(load_starting_map.before(Labels::Display))
                .with_system(set_game_state),
        );

        app.add_system_set(
            SystemSet::on_update(DisplayState::Game)
                .with_system(handle_esc)
                .with_system(handle_win)
                .with_system(handle_restart)
                .with_system(handle_box_highlight),
        );

        app.add_system_set(
            SystemSet::on_exit(DisplayState::Game)
                .label(Labels::ExitGame)
                .with_system(exit_to_main_menu)
                .with_system(reset_game_state.before(set_game_state)),
        );

        app.add_system_set(SystemSet::on_enter(DisplayState::Victory).with_system(setup_win))
            .add_system_set(
                SystemSet::on_update(DisplayState::Victory).with_system(handle_win_click),
            )
            .add_system_set(SystemSet::on_exit(DisplayState::Victory).with_system(delete_win));
        
        app.add_system_set(SystemSet::on_update(DisplayState::Restarting).with_system(reload.after(Labels::ExitGame)));
    }
}

fn set_game_state(mut game_state: ResMut<State<GameState>>) {
    game_state
        .set(GameState::Static)
        .expect("Could not set static game state");
}

fn reset_game_state(mut game_state: ResMut<State<GameState>>) {
    game_state
        .set(GameState::NotInGame)
        .expect("Could not reset game state");
}

fn handle_restart(mut keyboard_input: ResMut<Input<KeyCode>>, mut display_state: ResMut<State<DisplayState>>) {
    if keyboard_input.just_pressed(KeyCode::R) {
        display_state.set(DisplayState::Restarting).expect("Could not restart");
        keyboard_input.reset(KeyCode::R);
    }
}

fn reload(mut display_state: ResMut<State<DisplayState>>, mut timer: ResMut<RestartTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());         //workaround to avoid bugs related to switching states in ::on_enter or ::on_exit
    if timer.0.finished() {
        timer.0.reset();
        display_state.set(DisplayState::Game).expect("Could not reload game");
    }
}