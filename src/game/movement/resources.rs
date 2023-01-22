use bevy::prelude::*;

use crate::game::game_objects::{Direction, Position};

#[derive(Resource)]
pub struct AnimationTimer(pub Timer);

#[derive(Resource, Debug)]
pub struct MovementData {
    pub moved_positions: Vec<Position>,
    pub direction: Option<Direction>,
    pub positions_on_ice: Option<Vec<Position>>,
    pub is_on_ice: bool,
}
