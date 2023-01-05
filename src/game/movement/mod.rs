use crate::{labels::Labels, state::GameState};
use bevy::prelude::*;

use animation::{end_animation, move_animation};
use keyboard::handle_keypress;

use self::{events::MoveEvent, position_updating::handle_move};
use crate::game::game_objects::{Box, Player};

mod animation;
mod consts;
mod events;
mod keyboard;
mod position_updating;

pub type MovableInQuery = Or<(With<Box>, With<Player>)>;
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Moving).with_system(move_animation))
            .add_system_set(SystemSet::on_resume(GameState::Moving).with_system(move_animation))
            .add_system_set(SystemSet::on_update(GameState::Moving).with_system(move_animation))
            .add_system_set(SystemSet::on_exit(GameState::Moving).with_system(end_animation))
            .add_system_set(SystemSet::on_pause(GameState::Moving).with_system(end_animation));

        app.add_system_set(
            SystemSet::on_update(GameState::Static)
                .label(Labels::Movement)
                .with_system(handle_keypress.before(handle_move))
                .with_system(handle_move),
        );
        app.add_event::<MoveEvent>();
    }
}
