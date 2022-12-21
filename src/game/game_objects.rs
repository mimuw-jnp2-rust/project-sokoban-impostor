use bevy::prelude::{Component, Entity};

#[derive(Eq, Hash)]
pub enum GameObjects {
    Box(Option<Entity>),
    Wall,
    Empty,
}

#[derive(Component)]
pub struct Goal;

impl PartialEq for GameObjects {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Box(_), Self::Box(_)) => true, //we do not care about the insides of Box when comapring
            (Self::Wall, Self::Wall) => true,
            (Self::Empty, Self::Empty) => true,
            _ => false,
        }
    }
}

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Background;

#[derive(Component, PartialEq, Eq, Hash)]
pub struct Box;

#[derive(Component)]
pub struct Player {
    pub position: Position,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn neighbour(&self, dir: Direction) -> Position {
        match dir {
            Direction::Up => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::None => self.clone(),
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
