use bevy::{
    prelude::{Resource, Timer},
    utils::HashMap,
};

use crate::game::game_objects::GameObjects;
use crate::game::game_objects::Position;

#[derive(Resource)]
pub struct InputTimer(pub Timer);

#[derive(Resource)]
pub struct Board {
    pub entities: HashMap<Position, GameObjects>,
}

#[derive(Resource)]
pub struct Goals {
    pub goals: Vec<Position>,
}

#[derive(Resource)]
pub struct StartingPosition {
    pub position: Position,
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
