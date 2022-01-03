use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub mod events;
pub mod game;

pub const WINDOW_WIDTH: f32 = 1000.0;
pub const WINDOW_HEIGHT: f32 = 1000.0;
pub const ARENA_WIDTH: u32 = 100;
pub const ARENA_HEIGHT: u32 = 100;


#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Default)]
pub struct SnakeSegments(Vec<Entity>);

#[derive(Default)]
pub struct Wormholes(Vec<Entity>);

#[derive(Default)]
pub struct LastTailPosition(Option<Position>);

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SnakeMovement {
    Input,
    Movement,
    Eating,
    Growth,
}

pub struct Food;
pub struct Poison;
pub struct Wormhole;
pub struct SnakeSegment;

pub struct SnakeHead {
    direction: Direction,
    input_direction: Direction,
}

pub struct Materials {
    head_shape: shapes::RegularPolygon,
    segment_shape: shapes::RegularPolygon,
    food_shape: shapes::RegularPolygon,
    poison_shape: shapes::RegularPolygon,
    wormhole_shape: shapes::RegularPolygon,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}