use serde::Deserialize;
use uuid::Uuid;

use crate::game::player::PlayerManager;

use super::{condition::Condition, duration::Duration, status::Status, target::{Target, TargetId}, CardManager};


// Effect structure
#[derive(Deserialize, Debug, Clone)]
pub struct Effect {
    action: String,
    status: Option<String>,
    amount: Option<i32>,
    conditions: Option<Vec<Condition>>,
    source_target: Option<Target>,
    destination_target: Option<Target>,
    duration: Option<Duration>,
}

impl Effect {
    pub fn process(&self, player_id: &Uuid, card_instance_id: &Uuid, player_manager: &mut PlayerManager, card_manager: &mut CardManager) {
        // Check if the conditions are met
        // Match on the action
        // Individually check the targets

        //TODO FIX THIS 
        if self.conditions_met(player_id, card_instance_id, card_manager, &player_manager) {
            match self.action.as_str() {
                "freeze" => {
                    if let Some(target) = &self.source_target {
                        let resolved = target.resolve_id(player_id, card_instance_id, card_manager, player_manager);
                        match resolved {
                            TargetId::Player(target_player_id) => {
                                if let Some(mut_player) = player_manager.get_player_by_id_mut(&target_player_id) {
                                    mut_player.add_status_effect(Status::Frozen { duration: Duration::new("round".to_owned(), Some(1)) });
                                }
                            },
                            TargetId::Card {player_id: target_player_id, instance_id: target_card_instance_id} => {
                                if let Some(mut_card) = card_manager.get_card_from_instance_id_mut(&target_card_instance_id) {
                                    mut_card.add_status_effect(Status::Frozen { duration: Duration::new("round".to_owned(), Some(1)) });
                                }
                            },
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }
    }

    fn conditions_met(&self, player_id: &Uuid, card_instance_id: &Uuid, card_manager: &CardManager, player_manager: &PlayerManager) -> bool {
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
}