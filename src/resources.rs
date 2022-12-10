use bevy::{ prelude::{Resource, Timer}, utils::{Duration, HashMap}, };

use crate::game_objects::GameObjects;
use crate::game_objects::Position;

#[derive(Resource)]
pub struct InputTimer(pub Timer);


#[derive(Resource)]
pub struct Board {
    pub entities: HashMap<Position, GameObjects>,
}