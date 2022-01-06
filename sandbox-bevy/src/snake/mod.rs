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

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum FoodMovement {
    Movement,
}

pub struct Food;
pub struct Poison;
pub struct Wormhole;
pub struct SnakeSegment;

pub struct SnakeHead {
    direction: Direction,
    input_direction: Direction,
}

pub struct Shape {
    shape: shapes::RegularPolygon, 
    outline: Color,
    fill: Color,
}

pub struct Materials {
    snake_head: Shape,
    snake_segment: Shape,
    pizza: Handle<ColorMaterial>,
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

// Keeps the textures and materials for Arrows
// pub struct MaterialResource {
//     pizza_texture: Handle<ColorMaterial>,
// }

// impl FromWorld for MaterialResource {
//     fn from_world(world: &mut World) -> Self {
//         let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
//         let asset_server = world.get_resource::<AssetServer>().unwrap();

//         let pizza_handle = asset_server.load("images/neon-pizza-logo.png");
//         MaterialResource {
//             pizza_texture: materials.add(pizza_handle.into()),
//         }
//     }
// }