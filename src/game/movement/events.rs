use bevy::prelude::*;

use crate::game::game_objects::{Direction, Position};

pub struct MoveEvent {
    pub dir: Direction,
    pub entity: Entity,
    pub position: Position,
}