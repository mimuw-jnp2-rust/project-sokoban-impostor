use bevy::prelude::Component;

#[derive(PartialEq, Eq, Hash)]
pub enum GameObjects {
    Player,
    Box,
    Wall,
    Empty,
}

#[derive(Component)]
pub struct Player {
    pub position: Position,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub movable: bool,
    pub x: i32,
    pub y: i32
}

#[derive(Component, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
