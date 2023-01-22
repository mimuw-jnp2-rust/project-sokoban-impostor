use crate::{
    consts::MOVE_ANIMATION_TIME,
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

use super::display::{
    background::{render_board, render_border},
    despawn_board,
};

use resources::{AnimationTimer, MovementData};

mod animation;
pub mod consts;
mod events;
mod ice;
mod keyboard;
mod position_updating;
pub mod resources;
mod warp;

pub type MovableInQuery = Or<(With<Box>, With<Player>)>;
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState(Some(Move::Moving)))
                .with_system(handle_move.before(move_animation))
                .with_system(move_animation.before(handle_warp).before(handle_ice))
                .with_system(handle_warp.before(despawn_board))
                .with_system(handle_ice.before(despawn_board)) //otherwise it could ignore the positions_on_ice and end the animation
                .with_system(despawn_board.before(render_board).before(render_border))
                .with_system(render_board.before(continue_animation))
                .with_system(render_border.before(continue_animation))
                .with_system(continue_animation),
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
        app.insert_resource(AnimationTimer(Timer::from_seconds(
            MOVE_ANIMATION_TIME,
            TimerMode::Once,
        )));
        app.insert_resource(MovementData {
            positions_on_ice: None,
            moved_positions: Vec::new(),
            direction: None,
            is_on_ice: false,
        });
    }
}

fn continue_animation(
    mut movement_data: ResMut<MovementData>,
    mut app_state: ResMut<State<GameState>>,
    mut writer: EventWriter<MoveEvent>,
    mut timer: ResMut<AnimationTimer>,
) {
    if !timer.0.finished() {
        return;
    }
    let positions = movement_data.positions_on_ice.clone();
    if positions.is_none() {
        return;
    }
    let positions = positions.unwrap();
    if !positions.is_empty() {
        writer.send(MoveEvent {
            direction: movement_data.direction.expect("Movement missing direction"),
            positions, //this vector has less than 20 entries
        });
        movement_data.direction = None;
        movement_data.moved_positions.clear();
        movement_data.positions_on_ice = None;
        timer.0.reset();
    } else {
        app_state
            .set(GameState(Some(Move::Static)))
            .expect("Could not correctly finish movement animation");
    }
}
