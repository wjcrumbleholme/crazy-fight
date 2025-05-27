use std::{fs::{self}};
use serde::Deserialize;
use std::collections::HashMap;
use macroquad::prelude::*;

use super::GameManger;



pub struct CardManager {
    card_registry: HashMap<String, Card>
}

impl CardManager {
    pub fn new() -> Self {
        Self {
            card_registry: HashMap::new(),
        }
    }

    pub fn register_card(&mut self, card_path: &str) {
        let loaded_card = Card::load_from_file(card_path);
        self.card_registry.insert(loaded_card.get_id().clone(), loaded_card);
    }

    pub fn get_card(&self, id: &str) -> Option<&Card> {
        self.card_registry.get(id)
    }

}







#[derive(Deserialize, Clone)]
pub enum CardType {
    Character,
    Item,
    PowerUp,
    BattleCard,
    Weapon
}


#[derive(Deserialize, Clone)]
pub struct Card {
    name: String,
    id: String,
    card_type: CardType,
    img_path: String,
    description: String,
    ability: Option<Ability>
}

#[derive(Deserialize, Clone)]
pub struct Ability {
    id: String,
    params: Option<HashMap<String, i32>>,
}

impl Card {
    pub fn get_type(&self) -> &CardType {
        &self.card_type
    }
    pub fn get_img_path(&self) -> &String {
        &self.img_path
    }
    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn load_from_file(path: &str) -> Self {
        let data = fs::read_to_string(path).expect("Failed to read file");
        let card: Self = serde_json::from_str(&data).expect("Failed to parse JSON");

        println!("Loaded card {}", card.name);

        card
    }

}