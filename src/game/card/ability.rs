use serde::Deserialize;
use uuid::Uuid;

use crate::game::player::{Player, PlayerManager};

use super::{condition::Condition, effect::Effect, CardManager};

// Ability structure
#[derive(Deserialize, Debug, Clone)]
pub struct Ability {
    #[serde(rename = "type")]
    trigger: String,
    conditions: Option<Vec<Condition>>,
    effects: Vec<Effect>,
}

impl Ability {
    pub fn get_trigger(&self) -> &String{
        &self.trigger
    }

    pub fn conditions_met(&self, player_id: &Uuid, card_instance_id: &Uuid, card_manager: &CardManager, player_manager: &PlayerManager) -> bool {
        //If there are no conditions, treat as the conditions being met
        if let Some(conditions) = &self.conditions {
            for condition in conditions {
                if !condition.is_met(player_id, card_instance_id, card_manager, player_manager) {
                    return false;
                }
            }  
        }
        true
    }

    pub fn process_effects(&self, player_id: &Uuid, card_instance_id: &Uuid, player_manager: &mut PlayerManager, card_manager: &mut CardManager) {
        for effect in &self.effects {
            effect.process(player_id, card_instance_id, player_manager, card_manager)
        }
    }
}
