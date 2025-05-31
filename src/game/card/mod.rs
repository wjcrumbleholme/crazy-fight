use std::{fs::{self}};
use ability::Ability;
use serde::Deserialize;
use status::Status;
use uuid::Uuid;
use std::collections::HashMap;
use macroquad::prelude::*;



pub mod condition;
pub mod effect;
pub mod target;
pub mod ability;
pub mod status;
pub mod duration;


pub struct CardManager {
    card_registry: HashMap<String, Card>,
    instantiated_cards: HashMap<Uuid, Card>
}

impl CardManager {
    pub fn new() -> Self {
        Self {
            card_registry: HashMap::new(),
            instantiated_cards: HashMap::new()
        }
    }

    pub fn register_card(&mut self, card_path: &str) {
        let loaded_card = Card::load_from_file(card_path);
        self.card_registry.insert(loaded_card.get_card_id().clone(), loaded_card);
    }

    pub fn get_card(&self, id: &str) -> Option<&Card> {
        self.card_registry.get(id)
    }

    pub fn get_card_from_instance_id(&self, instance_id: &Uuid) -> Option<&Card> {
        self.instantiated_cards.get(instance_id)
    }

    pub fn get_card_from_instance_id_mut(&mut self, instance_id: &Uuid) -> Option<&mut Card> {
        self.instantiated_cards.get_mut(instance_id)
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
    #[serde(rename = "id")]
    card_id: String,
    #[serde(skip)]
    instance_id: Option<Uuid>,
    img_path: String,
    description: String,
    play_time: String,
    #[serde(skip)]
    is_active: Option<bool>,
    #[serde(skip)]
    status_effects: Vec<Status>,
    
}

impl BaseCard {
    fn card_id(&self) -> &String {
        &self.card_id
    }

    fn instance_id(&self) -> &Option<Uuid> {
        &self.instance_id
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
    #[serde(skip)]
    has_super_active: bool,
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




trait HasBase {
    fn base(&self) -> &BaseCard;
    fn mut_base(&mut self) -> &mut BaseCard;
}

impl HasBase for AddonCard {
    fn base(&self) -> &BaseCard {
        &self.base
    }

    fn mut_base(&mut self) -> &mut BaseCard {
        &mut self.base
    }
}

impl HasBase for BattleItemCard {
    fn base(&self) -> &BaseCard {
        &self.base
    }

    fn mut_base(&mut self) -> &mut BaseCard {
        &mut self.base
    }
}

impl HasBase for CharacterCard {
    fn base(&self) -> &BaseCard {
        &self.base
    }

    fn mut_base(&mut self) -> &mut BaseCard {
        &mut self.base
    }
}

impl HasBase for ItemCard {
    fn base(&self) -> &BaseCard {
        &self.base
    }

    fn mut_base(&mut self) -> &mut BaseCard {
        &mut self.base
    }
}
impl HasBase for SuperCharacterCard {
    fn base(&self) -> &BaseCard {
        &self.base
    }

    fn mut_base(&mut self) -> &mut BaseCard {
        &mut self.base
    }
}

impl HasBase for WeaponCard {
    fn base(&self) -> &BaseCard {
        &self.base
    }

    fn mut_base(&mut self) -> &mut BaseCard {
        &mut self.base
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

    pub fn get_card_id(&self) -> &String {
        match self {
            Card::Addon(c) => c.base().card_id(),
            Card::BattleItem(c) => c.base().card_id(),
            Card::Character(c) => c.base().card_id(),
            Card::Item(c) => c.base().card_id(),
            Card::SuperCharacter(c) => c.base().card_id(),
            Card::Weapon(c) => c.base().card_id(),
        }
    }

    pub fn get_instance_id(&self) -> &Option<Uuid> {
        match self {
            Card::Addon(c) => c.base().instance_id(),
            Card::BattleItem(c) => c.base().instance_id(),
            Card::Character(c) => c.base().instance_id(),
            Card::Item(c) => c.base().instance_id(),
            Card::SuperCharacter(c) => c.base().instance_id(),
            Card::Weapon(c) => c.base().instance_id(),
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

    pub fn get_ability(&self) -> &Option<Ability> {
        match self {
            Card::Addon(c) => &c.ability,
            Card::BattleItem(c) => &c.ability,
            Card::Character(c) => &c.ability,
            Card::Item(c) => &c.ability,
            Card::SuperCharacter(c) => &c.ability,
            Card::Weapon(c) => &None,
        }
    }

    pub fn add_status_effect(&mut self, effect: Status) {
        match self {
            Card::Addon(c) => c.mut_base().status_effects.push(effect),
            Card::BattleItem(c) => c.mut_base().status_effects.push(effect),
            Card::Character(c) => c.mut_base().status_effects.push(effect),
            Card::Item(c) => c.mut_base().status_effects.push(effect),
            Card::SuperCharacter(c) => c.mut_base().status_effects.push(effect),
            Card::Weapon(c) => c.mut_base().status_effects.push(effect)
        }
    }

}