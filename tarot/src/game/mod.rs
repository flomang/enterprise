use bevy::{
    prelude::*,
    reflect::TypeUuid,
};
use serde::Deserialize;
//use bevy_prototype_lyon::prelude::*;
pub mod tarot;


pub const WINDOW_WIDTH: f32 = 1000.0;
pub const WINDOW_HEIGHT: f32 = 1000.0;
pub const CARD_WIDTH: f32 = 129.0;
pub const CARD_HEIGHT: f32 = 129.0;

pub struct CardFlipEvent {
    pub entity: Entity,
}

#[derive(Default)]
pub struct Cards(Vec<Entity>);

#[derive(Default)]
pub struct Shoe(Vec<usize>);

#[derive(Component)]
pub struct Rect{
    x: f32,
    y:f32,
    width: f32,
    height: f32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CardState{
    Down,
    Up,
    TransitionDown,
    TransitionUp,
}

#[derive(Component)]
pub struct Card{
    flipped: bool,
    state: CardState, 
    rect: Rect,
}

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct CatalogAsset {
    title: String,
    cards: Vec<CardAsset>,
}

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct CardAsset {
    order: u32,
    title: String,
    up: String,
    reverse: String,
}

pub struct Materials {
    sprite_sheet: Handle<TextureAtlas>,
    card_catalog: Handle<CatalogAsset>,
}