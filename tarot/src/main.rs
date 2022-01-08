use bevy::prelude::*;
use std::path::Path;
use std::fs::File;
use serde::Deserialize;
use rand::seq::SliceRandom; // 0.7.2


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
    let json_file_path = Path::new("assets/cards/modern-magick.json");
    let file = File::open(json_file_path).expect("file not found");
    let cards: Vec<Card> = serde_json::from_reader(file).expect("error while reading");
    let random_card = cards.choose(&mut rand::thread_rng()).unwrap();

    print!("({}) {} - ", random_card.order, random_card.title);
    if rand::random() {
        println!("up\t\n {}", random_card.up);
    } else {
        println!("reversed\t\n {}", random_card.reverse);

    }
}
