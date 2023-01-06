use crate::game::game_objects::{Direction, Position};

pub struct MoveEvent {
    pub direction: Direction,
    pub positions: Vec<Position>,
}
