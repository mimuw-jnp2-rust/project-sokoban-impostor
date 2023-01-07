use crate::{
    labels::Labels,
    state::{GameState, Move},
};
use bevy::prelude::*;

use animation::{end_animation, move_animation};
use events::MoveEvent;
use ice::handle_ice;
use keyboard::handle_keypress;
use position_updating::handle_move;
use warp::handle_warp;

use crate::game::game_objects::{Box, Player};

mod animation;
mod consts;
mod events;
mod ice;
mod keyboard;
mod position_updating;
mod warp;

pub type MovableInQuery = Or<(With<Box>, With<Player>)>;
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState(Some(Move::Moving)))
                .with_system(handle_move.before(move_animation))
                .with_system(move_animation.before(handle_warp))
                .with_system(handle_warp.before(handle_ice))
                .with_system(handle_ice)
        )
        .add_system_set(
            SystemSet::on_exit(GameState(Some(Move::Moving))).with_system(end_animation),
        );

        app.add_system_set(
            SystemSet::on_update(GameState(Some(Move::Static)))
                .label(Labels::Movement)
                .with_system(handle_keypress),
        );
        app.add_event::<MoveEvent>();
    }
}
