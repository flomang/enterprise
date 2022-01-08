//use std::collections::HashMap;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub const WINDOW_WIDTH: f32 = 1000.0;
pub const WINDOW_HEIGHT: f32 = 1000.0;
pub const ARENA_WIDTH: u32 = 100;
pub const ARENA_HEIGHT: u32 = 100;

pub mod tarot;


struct Card;

pub struct Materials {
    card_cover: Handle<ColorMaterial>,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    x: i32,
    y: i32,
}

//pub struct Card<'a> {
//    order: u8,
//    title: &'a str,
//    up: &'a str,
//    reverse: &'a str,
//}
//
//impl<'a> Card<'a> {
//    pub fn new(order: u8, title: &'a str, up: &'a str, reverse: &'a str) -> Card<'a> {
//        Card{
//            order: order,
//            title: title,
//            up: up,
//            reverse: reverse,
//        }
//    }
//}
//
//pub struct MajorArcana<'a> {
//    card_titles: HashMap<u8, &'a str>,
//}
//
//impl<'a> MajorArcana<'a> {
//    pub fn new() -> MajorArcana<'a> {
//        MajorArcana{
//            card_titles: build_deck(),
//        }
//    }
//
//    // TODO sort by order
//    pub fn sorted_keys(&self) -> Vec<&u8> {
//        let mut orders: Vec<&u8> = self.card_titles.keys().collect();
//        orders.sort();
//        orders
//    }
//
//    pub fn card_title(&self, order: u8) -> &str {
//        self.card_titles[&order]
//    }
//}
//
//
//fn build_deck<'a>() -> HashMap<u8, &'a str> {
//    let mut cards: HashMap<u8, &str> = HashMap::new();
//    cards.insert(0, "The Fool");
//    cards.insert(1, "The Magician");
//    cards.insert(2, "The High Priestess");
//    cards.insert(3, "The Empress");
//    cards.insert(4, "The Emperor");
//    cards.insert(5, "The Hierophant");
//    cards.insert(6, "The Lovers");
//    cards.insert(7, "The Chariot");
//    cards.insert(8, "Strength");
//    cards.insert(9, "The Hermit");
//    cards.insert(10, "Wheel of Fortune");
//    cards.insert(11, "Justice");
//    cards.insert(12, "The Hanged Man");
//    cards.insert(13, "Death");
//    cards.insert(14, "Temperance");
//    cards.insert(15, "Devil");
//    cards.insert(16, "The Tower");
//    cards.insert(17, "The Star");
//    cards.insert(18, "The Moon");
//    cards.insert(19, "The Sun");
//    cards.insert(20, "Judgement");
//    cards.insert(21, "The Universe");
//    cards
//}