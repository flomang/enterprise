use bevy::prelude::*;
//use rand::seq::SliceRandom;
//use serde::Deserialize;
//use std::fs::File;
//use std::path::Path; // 0.7.2

mod game;
//
//#[derive(Debug, Deserialize)]
//#[serde(rename_all = "camelCase")]
//struct Card {
//    order: u8,
//    title: String,
//    up: String,
//    reverse: String,
//}

fn main() {
    // let json_file_path = Path::new("assets/cards/modern-magick.json");
    // let file = File::open(json_file_path).expect("file not found");
    // let cards: Vec<Card> = serde_json::from_reader(file).expect("error while reading");
    // let random_card = cards.choose(&mut rand::thread_rng()).unwrap();

    // print!("({}) {} - ", random_card.order, random_card.title);
    // if rand::random() {
    //     println!("up\t\n {}", random_card.up);
    // } else {
    //     println!("reversed\t\n {}", random_card.reverse);
    // }

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Tarot".to_string(),
            width: game::WINDOW_WIDTH,
            height: game::WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(game::tarot::setup)
        .add_system(game::tarot::animate_sprite_system)
        // .add_startup_stage(
        //      "game_setup",
        //      SystemStage::single(game::tarot::spawn_card.system()),
        // )
        .run();
}

