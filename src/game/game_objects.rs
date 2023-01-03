use bevy::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum GameObjects {
    Box,
    Wall,
    Empty,
    Player,
}

#[derive(Component)]
pub struct Goal;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Background;

#[derive(Component, PartialEq, Eq, Hash)]
pub struct Box;

#[derive(Component)]
pub struct Player;

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
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
