use bevy::prelude::*;

mod game;

fn main() {
    let tarot_cards = game::MajorArcana::new();

    for o in tarot_cards.sorted_keys() {
        println!("({}) {}", o, tarot_cards.card_title(*o as i32));
    }
    println!("({}) {}", 0, tarot_cards.card_title(0))
}
