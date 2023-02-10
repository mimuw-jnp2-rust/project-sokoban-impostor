use std::cmp::Ordering;

use bevy::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum GameObject {
    Box,
    Wall,
    HidingWall,
    Empty,
    Player,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Floor {
    HiddenWall,
    Tile,
    Ice,
    Goal,
    Warp(usize),
    Button,
}

#[derive(Component)]
pub struct Button {
    pub on: bool,
}

#[derive(Component)]
pub struct HiddenWall;

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

#[derive(Component)]
pub struct BoxButton;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub map: usize,
}

impl Position {
    pub fn next_position(&self, dir: Direction) -> Position {
        match dir {
            Direction::Up => Position {
                x: self.x,
                y: self.y + 1,
                map: self.map,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y - 1,
                map: self.map,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
                map: self.map,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
                map: self.map,
            },
        }
    }

    pub fn previous_position(&self, dir: Direction) -> Position {
        match dir {
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
                map: self.map,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
                map: self.map,
            },
            Direction::Left => Position {
                x: self.x + 1,
                y: self.y,
                map: self.map,
            },
            Direction::Right => Position {
                x: self.x - 1,
                y: self.y,
                map: self.map,
            },
        }
    }

    pub fn cmp_to_other(&self, other: &Self, dir: Direction) -> Ordering {
        match dir {
            Direction::Up => {
                if self.y > other.y {
                    Ordering::Less
                }
                else if self.y == other.y {
                    Ordering::Equal
                }
                else {
                    Ordering::Greater
                }
            },
            Direction::Down => {
                if self.y > other.y {
                    Ordering::Greater
                }
                else if self.y == other.y {
                    Ordering::Equal
                }
                else {
                    Ordering::Less
                }
            },
            Direction::Left => {
                if self.x < other.x {
                    Ordering::Less
                }
                else if self.x == other.x {
                    Ordering::Equal
                }
                else {
                    Ordering::Greater
                }
            },
            Direction::Right => {
                if self.x > other.x {
                    Ordering::Less
                }
                else if self.x == other.x {
                    Ordering::Equal
                }
                else {
                    Ordering::Greater
                }
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
