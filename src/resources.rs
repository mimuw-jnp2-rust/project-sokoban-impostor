use bevy::{prelude::*, utils::HashMap};

use crate::consts::*;
use crate::game::game_objects::{Direction, Position};
use crate::game::game_objects::{Floor, GameObject};

#[derive(Resource)]
pub struct InputTimer(pub Timer);

#[derive(Resource)]
pub struct VictoryTimer(pub Timer);

#[derive(Resource)]
pub struct Board {
    entities: HashMap<Position, Entity>,
    objects: HashMap<Position, GameObject>,
    floors: HashMap<Position, Floor>,
    goals: Vec<Position>,
    player_position: Position,
}

impl Board {
    pub fn new() -> Self {
        Board {
            entities: HashMap::new(),
            objects: HashMap::new(),
            floors: HashMap::new(),
            goals: Vec::new(),
            player_position: Position { x: 0, y: 0 },
        }
    }

    pub fn get_player_position(&self) -> Position {
        self.player_position
    }

    pub fn get_entity(&self, position: Position) -> Entity {
        *self
            .entities
            .get(&position)
            .expect("Tried searching entity of invalid position")
    }

    pub fn get_object_type(&self, position: Position) -> GameObject {
        *self.objects.get(&position).unwrap_or(&GameObject::Empty)
    }

    pub fn get_floor_type(&self, position: Position) -> Floor {
        *self.floors.get(&position).unwrap_or(&Floor::Tile)
    }

    pub fn get_goals(&self) -> Vec<Position> {
        self.goals.clone() //realistically, this vector won't exceed 20 entries
    }

    pub fn insert_object(&mut self, position: Position, object: GameObject) {
        self.objects.insert(position, object);
        if object == GameObject::Player {
            self.player_position = position;
        }
    }

    pub fn insert_entity(&mut self, position: Position, entity: Entity) {
        self.entities.insert(position, entity);
    }

    pub fn insert_floor(&mut self, position: Position, floor: Floor) {
        self.floors.insert(position, floor);
        if floor == Floor::Goal {
            self.goals.push(position);
        }
    }

    pub fn move_object(&mut self, position: Position, dir: Direction) {
        let object = self
            .objects
            .remove(&position)
            .expect("Tried to move nothing");
        if object == GameObject::Player {
            self.player_position = position.neighbour(dir);
        }
        self.objects.insert(position.neighbour(dir), object);
        let entity = self
            .entities
            .remove(&position)
            .expect("Entity not in board");
        self.entities.insert(position.neighbour(dir), entity);
    }

    pub fn clear(&mut self) {
        self.entities.clear();
        self.objects.clear();
        self.floors.clear();
        self.goals.clear();
    }
}

#[derive(Resource)]
pub struct MovementData {
    pub moved_positions: Vec<Position>,
    pub direction: Option<Direction>,
}

#[derive(Resource)]
pub struct MapSize {
    pub height: u32,
    pub width: u32,
}

#[derive(Resource)]
pub struct CurrentLevel {
    pub level_number: usize,
    pub level_map_str: &'static str,
}

#[derive(Resource)]
pub struct Images {
    pub goal_image: Handle<Image>,
    pub player_image: Handle<Image>,
    pub box_image: Handle<Image>,
    pub box_on_goal_image: Handle<Image>,
    pub wall_image: Handle<Image>,
    pub tile_image: Handle<Image>,
    pub ice_image: Handle<Image>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("Asset server not found in world");
        let player_image = asset_server.load(PLAYER_TEXTURE);
        let box_image = asset_server.load(BOX_TEXTURE);
        let wall_image = asset_server.load(WALL_TEXTURE);
        let goal_image = asset_server.load(GOAL_TEXTURE);
        let box_on_goal_image = asset_server.load(BOX_ON_GOAL_TEXTURE);
        let tile_image = asset_server.load(TILE_TEXTURE);
        let ice_image = asset_server.load(ICE_TEXTURE);

        Images {
            player_image,
            box_image,
            wall_image,
            goal_image,
            box_on_goal_image,
            tile_image,
            ice_image,
        }
    }
}
