use std::{fs::{self}};
use serde::Deserialize;
use std::collections::HashMap;
use macroquad::prelude::*;

use super::{condition::Condition, player::{self, Player}};



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


#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "card_type")]
pub enum Card {
    Character(CharacterCard),
    SuperCharacter(SuperCharacterCard),
    Item(ItemCard),
    Addon(AddonCard),
    BattleItem(BattleItemCard),
    Weapon(WeaponCard),
}


// Shared properties of all cards
#[derive(Deserialize, Debug, Clone)]
struct BaseCard {
    name: String,
    id: String,
    img_path: String,
    description: String,
    play_time: String,
    is_active: Option<bool>,
}

impl BaseCard {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_img_path(&self) -> &String {
        &self.img_path
    }
}


// Character card
#[derive(Deserialize, Debug, Clone)]
pub struct CharacterCard {
    #[serde(flatten)]
    base: BaseCard,
    damage: Option<i32>,
    super_char_id: Option<String>,
    ability: Option<Ability>,
}

// Super Character card
#[derive(Deserialize, Debug, Clone)]
pub struct SuperCharacterCard {
    #[serde(flatten)]
    base: BaseCard,
    additional_damage: Option<i32>,
    ability: Option<Ability>,
}

// Item card
#[derive(Deserialize, Debug, Clone)]
pub struct ItemCard {
    #[serde(flatten)]
    base: BaseCard,
    ability: Option<Ability>,
}

// Battle Item card
#[derive(Deserialize, Debug, Clone)]
pub struct BattleItemCard {
    #[serde(flatten)]
    base: BaseCard,
    ability: Option<Ability>,
}

// Weapon card
#[derive(Deserialize, Debug, Clone)]
pub struct WeaponCard {
    #[serde(flatten)]
    base: BaseCard,
    damage: Option<i32>,
    synergy_card_id: Option<String>,
    synergy_damage: Option<i32>,
}

// Addon card
#[derive(Deserialize, Debug, Clone)]
pub struct AddonCard {
    #[serde(flatten)]
    base: BaseCard,
    reveal_time: Option<String>,
    ability: Option<Ability>,
}


// Ability structure
#[derive(Deserialize, Debug, Clone)]
pub struct Ability {
    #[serde(rename = "type")]
    ability_type: String,
    trigger: Option<String>,
    conditions: Option<Vec<Condition>>,
    effects: Vec<Effect>,
}

impl Ability {
    pub fn get_trigger(&self) -> Option<String>{
        self.trigger.clone()
    }

    pub fn conditions_met(&self, player: &Player) -> bool {
        //If there are no conditions, treat as the conditions being met
        if let Some(conditions) = &self.conditions {
            for condition in conditions {
                if !condition.is_met(player) {
                    return false;
                }
            }  
        }
        true
    }
}

// Effect structure
#[derive(Deserialize, Debug, Clone)]
struct Effect {
    action: String,
    status: Option<String>,
    amount: Option<i32>,
    conditions: Option<Vec<Condition>>,
    source_target: Option<Target>,
    destination_target: Option<Target>,
}

// Target structure
#[derive(Deserialize, Debug, Clone)]
struct Target {
    #[serde(rename = "type")]
    target_type: String,
    owner: Option<String>,
    card: Option<String>,
}




trait HasBase {
    fn base(&self) -> &BaseCard;
}

impl HasBase for AddonCard {
    fn base(&self) -> &BaseCard {
        &self.base
    }
}

impl HasBase for BattleItemCard {
    fn base(&self) -> &BaseCard {
        &self.base
    }
}

impl HasBase for CharacterCard {
    fn base(&self) -> &BaseCard {
        &self.base
    }
}

impl HasBase for ItemCard {
    fn base(&self) -> &BaseCard {
        &self.base
    }
}
impl HasBase for SuperCharacterCard {
    fn base(&self) -> &BaseCard {
        &self.base
    }
}

impl HasBase for WeaponCard {
    fn base(&self) -> &BaseCard {
        &self.base
    }
}






fn remove_filename(path: &str) -> &str {
    match path.rfind('/') {
        Some(index) => &path[..index],
        None => path, // no slash found, return original
    }
}

impl Card {
    pub fn load_from_file(path: &str) -> Self {
        let data = fs::read_to_string(path).expect("Failed to read file");
        let mut card: Self = serde_json::from_str(&data).expect("Failed to parse JSON");

        let base_path = remove_filename(path).to_owned();

        match &mut card {
            Card::Addon(c) => {
                c.base.img_path = format!("{}/{}", base_path, c.base().img_path);
                println!("Loaded card {}", c.base().name);
            },
            Card::BattleItem(c) => {
                c.base.img_path = format!("{}/{}", base_path, c.base().img_path);
                println!("Loaded card {}", c.base().name);
            },
            Card::Character(c) => {
                c.base.img_path = format!("{}/{}", base_path, c.base().img_path);
                println!("Loaded card {}", c.base().name);
            },
            Card::Item(c) => {
                c.base.img_path = format!("{}/{}", base_path, c.base().img_path);
                println!("Loaded card {}", c.base().name);
            },
            Card::SuperCharacter(c) => {
                c.base.img_path = format!("{}/{}", base_path, c.base().img_path);
                println!("Loaded card {}", c.base().name);
            },
            Card::Weapon(c) => {
                c.base.img_path = format!("{}/{}", base_path, c.base().img_path);
                println!("Loaded card {}", c.base().name);
            },
        }

        card
    }

    pub fn get_id(&self) -> String {
        match self {
            Card::Addon(c) => c.base().get_id(),
            Card::BattleItem(c) => c.base().get_id(),
            Card::Character(c) => c.base().get_id(),
            Card::Item(c) => c.base().get_id(),
            Card::SuperCharacter(c) => c.base().get_id(),
            Card::Weapon(c) => c.base().get_id(),
        }
    }

    pub fn get_img_path(&self) -> &String {
        match self {
            Card::Addon(c) => c.base().get_img_path(),
            Card::BattleItem(c) => c.base().get_img_path(),
            Card::Character(c) => c.base().get_img_path(),
            Card::Item(c) => c.base().get_img_path(),
            Card::SuperCharacter(c) => c.base().get_img_path(),
            Card::Weapon(c) => c.base().get_img_path(),
        }
    }

    pub fn get_ability(&self) -> Option<Ability> {
        match self {
            Card::Addon(c) => c.ability.clone(),
            Card::BattleItem(c) => c.ability.clone(),
            Card::Character(c) => c.ability.clone(),
            Card::Item(c) => c.ability.clone(),
            Card::SuperCharacter(c) => c.ability.clone(),
            Card::Weapon(c) => None,
        }
    }

}