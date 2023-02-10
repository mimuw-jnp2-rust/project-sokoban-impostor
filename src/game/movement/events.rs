use crate::game::game_objects::{Direction, Position, Floor, GameObject};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct EnteredFloorEvent {
    pub floor: Floor,
    pub position: Position,
    pub object: GameObject,
    pub direction: Direction,
}

pub struct ExitedFloorEvent {
    pub floor: Floor,
    pub position: Position,
    pub object: GameObject,
    pub direction: Direction,
}