use std::{fs::{self, read_to_string, File}, path::Path};
use serde::Deserialize;
use std::collections::HashMap;
use macroquad::prelude::*;

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
    pub name: String,
    pub card_type: CardType,
    pub img_path: String,
    pub description: String,
    pub ability: Option<Ability>
}

#[derive(Deserialize, Clone)]
pub struct Ability {
    id: String,
    params: Option<HashMap<String, i32>>,
}

impl Card {
    pub fn get_type(&self) -> CardType {
        self.card_type.clone()
    }
    pub fn get_img_path(&self) -> String {
        self.img_path.clone()
    }
    pub fn load_from_file(path: &str) -> Self {
        let data = fs::read_to_string(path).expect("Failed to read file");
        let card: Self = serde_json::from_str(&data).expect("Failed to parse JSON");

        println!("Loaded card {}", card.name);

        card
    }

}