use bevy::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum GameObject {
    Box,
    Wall,
    Empty,
    Player,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Floor {
    Tile,
    Ice,
    Goal,
    Warp(usize),
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

#[derive(Component)]
pub struct Ice;

#[derive(Component)]
pub struct Warp;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn next_position(&self, dir: Direction) -> Position {
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

    pub fn previous_position(&self, dir: Direction) -> Position {
        match dir {
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
