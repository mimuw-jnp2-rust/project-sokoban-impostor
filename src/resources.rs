use bevy::{prelude::*, utils::HashMap};

use crate::consts::*;
use crate::game::game_objects::{Direction, Position};
use crate::game::game_objects::{Floor, GameObject};

#[derive(Resource)]
pub struct InputTimer(pub Timer);

#[derive(Resource)]
pub struct VictoryTimer(pub Timer);

#[derive(Resource)]
pub struct RestartTimer(pub Timer);

#[derive(Copy, Clone, Debug)]
pub struct MapSize {
    pub height: u32,
    pub width: u32,
}

#[derive(Debug)]
struct SingleBoard {
    entities: HashMap<Position, Entity>,
    objects: HashMap<Position, GameObject>,
    floors: HashMap<Position, Floor>,
    goals: Vec<Position>,
    map_size: MapSize,
    player_position: Position,
    warp_positions: [Position; MAX_MAPS]
}

#[derive(Resource, Debug)]
pub struct Board {
    boards: Vec<SingleBoard>,
    current: usize,
}

impl Board {
    pub fn new() -> Self {
        let mut boards = Vec::new();
        for map in 0..MAX_MAPS {
            boards.push(SingleBoard {
                entities: HashMap::new(),
                objects: HashMap::new(),
                floors: HashMap::new(),
                goals: Vec::new(),
                map_size: MapSize {
                    width: 0,
                    height: 0,
                },
                player_position: Position { x: 0, y: 0, map },
                warp_positions: [Position { x: 0, y: 0, map}; 10],
            });
        }
        Board {
            current: INITIAL_MAP,
            boards,
        }
    }

    pub fn set_current_map(&mut self, current: usize) {
        self.current = current;
    }

    pub fn set_map_size(&mut self, map_size: MapSize) {
        self.boards[self.current].map_size = map_size;
    }

    pub fn get_map_size(&self) -> MapSize {
        self.boards[self.current].map_size
    }

    pub fn get_player_position(&self) -> Position {
        self.boards[self.current].player_position
    }

    pub fn get_entity(&self, position: Position) -> Entity {
        if position.map != self.current {
            panic!("Asking for entity from other map");
        }
        *self.boards[self.current]
            .entities
            .get(&position)
            .expect("Tried searching entity of invalid position")
    }

    pub fn get_object_type(&self, position: Position) -> GameObject {
        *self.boards[position.map]
            .objects
            .get(&position)
            .unwrap_or(&GameObject::Empty)
    }

    pub fn get_floor_type(&self, position: Position) -> Floor {
        *self.boards[position.map]
            .floors
            .get(&position)
            .unwrap_or(&Floor::Tile)
    }

    pub fn get_all_goals(&self) -> Vec<Position> {
        let mut goals_vec = Vec::new();
        for map in 0..MAX_MAPS {
            goals_vec.push(self.boards[map].goals.clone());
        }
        goals_vec.concat() //realistically, this vector won't exceed 20 entries so cloning isn't a problem
    }

    pub fn get_current_goals(&self) -> Vec<Position> {
        self.boards[self.current].goals.clone()
    }

    pub fn get_current_map(&self) -> usize {
        self.current
    }

    pub fn insert_object(&mut self, position: Position, object: GameObject) {
        self.boards[position.map].objects.insert(position, object);
        if object == GameObject::Player {
            self.boards[position.map].player_position = position;
        }
    }

    pub fn insert_entity(&mut self, position: Position, entity: Entity) {
        self.boards[position.map].entities.insert(position, entity);
    }

    pub fn insert_floor(&mut self, position: Position, floor: Floor) {
        self.boards[position.map].floors.insert(position, floor);
        if floor == Floor::Goal {
            self.boards[position.map].goals.push(position);
        }
        if let Floor::Warp(map) = floor {
            self.boards[position.map].warp_positions[map] = position;
        }
    }

    pub fn move_object(&mut self, position: Position, dir: Direction) {
        let object = self.boards[position.map]
            .objects
            .remove(&position)
            .expect("Tried to move nothing");
        if object == GameObject::Player {
            self.boards[position.map].player_position = position.next_position(dir);
        }
        self.boards[position.map]
            .objects
            .insert(position.next_position(dir), object);
        if position.map == self.current {
            let entity = self.boards[position.map]
                .entities
                .remove(&position)
                .expect("Entity not in board");
            self.boards[position.map]
                .entities
                .insert(position.next_position(dir), entity);
        }
    }

    pub fn delete_object(&mut self, position: Position) -> Entity {
        self.boards[position.map].objects.remove(&position).expect("Could not remove object");
        self.boards[position.map].entities.remove(&position).expect("Could not remove entity")
    }

    pub fn get_warp_position(&self, from: usize, to: usize) -> Position {
        self.boards[from].warp_positions[to]
    }

    pub fn clear(&mut self) {
        for map in 0..MAX_MAPS {
            self.boards[map].entities.clear();
            self.boards[map].objects.clear();
            self.boards[map].floors.clear();
            self.boards[map].goals.clear();
        }
    }
}

#[derive(Resource)]
pub struct MovementData {
    pub moved_positions: Vec<Position>,
    pub direction: Option<Direction>,
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
    pub warp_image: Handle<Image>,
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
        let warp_image = asset_server.load(WARP_TEXTURE);

        Images {
            player_image,
            box_image,
            wall_image,
            goal_image,
            box_on_goal_image,
            tile_image,
            ice_image,
            warp_image,
        }
    }
}
