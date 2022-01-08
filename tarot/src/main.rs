use bevy::prelude::*;
use std::path::Path;
use std::fs::File;
use serde::Deserialize;

mod game;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Card {
    order: u8, 
    title: String,
    up: String,
    reverse: String
}

fn main() {
    //let tarot_cards = game::MajorArcana::new();

    //for o in tarot_cards.sorted_keys() {
    //    println!("({}) {}", o, tarot_cards.card_title(*o));
    //}
    //println!("({}) {}", 0, tarot_cards.card_title(0));

    let json_file_path = Path::new("assets/cards/modern-magick.json");
    let file = File::open(json_file_path).expect("file not found");
    let cards: Vec<Card> = serde_json::from_reader(file).expect("error while reading");
   for card in cards {
       println!("({}) {}", card.order, card.title)
   }
}
