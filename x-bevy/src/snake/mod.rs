use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub mod events;
pub mod game;

pub const WINDOW_WIDTH: f32 = 1000.0;
pub const WINDOW_HEIGHT: f32 = 1000.0;
pub const ARENA_WIDTH: u32 = 100;
pub const ARENA_HEIGHT: u32 = 100;

#[derive(Default, Component, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Default)]
pub struct SnakeSegments(Vec<Entity>);

#[derive(Default)]
pub struct LastTailPosition(Option<Position>);

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SnakeMovement {
    Input,
    Movement,
    Eating,
    Growth,
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum FoodState {
    Movement,
    Spawn,
}

#[derive(Component)]
pub struct Food;
#[derive(Component)]
pub struct Poison;
#[derive(Component)]
pub struct SnakeSegment;

#[derive(Component)]
pub struct SnakeHead {
    direction: Direction,
    input_direction: Direction,
}

#[derive(Component)]
pub struct Shape {
    shape: shapes::RegularPolygon, 
    outline: Color,
    fill: Color,
}

pub struct Materials {
    snake_head: Shape,
    snake_segment: Shape,
    pizza_handle: Handle<Image>,
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