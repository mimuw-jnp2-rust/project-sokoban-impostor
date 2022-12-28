use bevy::{prelude::*, utils::HashMap};

use crate::game::game_objects::GameObjects;
use crate::game::game_objects::{Position, Direction};

#[derive(Resource)]
pub struct InputTimer(pub Timer);

#[derive(Resource)]
pub struct Board {
    pub entities: HashMap<Position, GameObjects>,
    pub player_position: Position,
}

#[derive(Resource)]
pub struct Goals {
    pub goals: Vec<Position>,
}

#[derive(Resource)]
pub struct MovementData {
    pub data: Option<MovementEntities>
}

pub struct MovementEntities {
    pub boxes_data: Vec<(Entity, Position)>,
    pub player_data: (Entity, Position),
    pub direction: Direction,
}

#[derive(Resource)]
pub struct MapSize {
    pub height: u32,
    pub width: u32,
}

#[derive(Resource)]
pub struct CurrentLevel {
    pub level_number: u32,
    pub level_map_str: &'static str,
}
