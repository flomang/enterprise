use std::collections::HashMap;

pub struct Card {
    order: u8,
    title: String,
    up: String,
    reverse: String,
}

impl Card {
    pub fn new(order: u8, title: String, up: String, reverse: String) -> Card {
        Card{
            order: order,
            title: title,
            up: up,
            reverse: reverse,
        }
    }
}

pub struct MajorArcana {
    hash_map: HashMap<u8, String>,
    cards: HashMap<u8, Card>,
}

impl MajorArcana {
    pub fn new() -> MajorArcana {
        MajorArcana{
            hash_map: build_deck(),
            cards: HashMap::new(),
        }
    }

    // TODO sort by order
    pub fn sorted_keys(&self) -> Vec<&u8> {
        let mut orders: Vec<&u8> = self.hash_map.keys().collect();
        orders.sort();
        orders
    }

    pub fn card_title(&self, order: u8) -> &String {
        let title = &self.hash_map[&order];
        title
    }

    pub fn add_definition(&mut self, order: u8, up: String, reverse: String) {
        let title = self.card_title(order);
        let card = Card::new(order, title.clone(), up, reverse);
        self.cards.insert(order, card);
    }
}


fn build_deck() -> HashMap<u8, String> {
    let mut cards: HashMap<u8, String> = HashMap::new();
    cards.insert(0, String::from("The Fool"));
    cards.insert(1, String::from("The Magician"));
    cards.insert(2, String::from("The High Priestess"));
    cards.insert(3, String::from("The Empress"));
    cards.insert(4, String::from("The Emperor"));
    cards.insert(5, String::from("The Hierophant"));
    cards.insert(6, String::from("The Lovers"));
    cards.insert(7, String::from("The Chariot"));
    cards.insert(8, String::from("Strength"));
    cards.insert(9, String::from("The Hermit"));
    cards.insert(10, String::from("Wheel of Fortune"));
    cards.insert(11, String::from("Justice"));
    cards.insert(12, String::from("The Hanged Man"));
    cards.insert(13, String::from("Death"));
    cards.insert(14, String::from("Temperance"));
    cards.insert(15, String::from("Devil"));
    cards.insert(16, String::from("The Tower"));
    cards.insert(17, String::from("The Star"));
    cards.insert(18, String::from("The Moon"));
    cards.insert(19, String::from("The Sun"));
    cards.insert(20, String::from("Judgement"));
    cards.insert(21, String::from("The Universe"));
    cards
}