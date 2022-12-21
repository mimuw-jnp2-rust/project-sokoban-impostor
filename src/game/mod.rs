use crate::display::{setup_background, setup_move};
use crate::maps::load_starting_map;
use crate::movement::keyboard_input_system;
use crate::state::GameState;
use bevy::prelude::*;

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
        .add_system_set(SystemSet::on_update(GameState::Game).with_system(keyboard_input_system));
    }
}
