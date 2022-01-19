use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

pub mod tarot;
pub mod menu;
pub mod splash;

pub const WINDOW_WIDTH: f32 = 1000.0;
pub const WINDOW_HEIGHT: f32 = 1000.0;
pub const CARD_WIDTH: f32 = 129.0;
pub const CARD_HEIGHT: f32 = 129.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    Paused,
}

pub struct CardFlipEvent {
    pub entity: Entity,
}

#[derive(Default)]
pub struct Cards(Vec<Entity>);

#[derive(Default)]
pub struct Shoe(Vec<usize>);

#[derive(Component)]
pub struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rect {
    pub fn contains(&self, x: f32, y:f32) -> bool {
        let x1 = self.x - self.width;
        let y1 = self.y - self.height;
        let x2 = self.x + self.width;
        let y2 = self.y + self.height;

        if x > x1 && x < x2 && y > y1 && y < y2 {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CardState {
    Down,
    Up,
    FlipDown,
    FlipUp,
    TransitionDown,
    TransitionUp,
}

#[derive(Component)]
pub struct Card {
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

// pub struct Materials {
//     sprite_sheet: Handle<TextureAtlas>,
//     card_catalog: Handle<CatalogAsset>,
// }


// This example will display a simple menu using Bevy UI where you can start a new game,
// change some settings or quit. There is no actual game, it will just display the current
// settings for 5 seconds before going back to the menu.

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    Menu,
    Game,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
