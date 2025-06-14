use serde::Deserialize;
use uuid::Uuid;

use super::super::player::PlayerManager;

use super::CardManager;



// Target structure
#[derive(Deserialize, Debug, Clone)]
pub struct Target {
    #[serde(rename = "type")]
    target_type: String,
    owner: Option<String>,
    card: Option<String>,
}

pub enum TargetId {
    Player(Uuid),
    Card {player_id: Uuid, instance_id: Uuid},
    None
}


// If target_type is player, then return a ResolvedTarget(PlayerId) 
// If target_type is card, then return a ResolvedTarget(PlayerId, InstanceId)
// All this has to do is return Ids, not actual data.



impl Target {
    pub fn resolve_id(&self, acting_player_id: &Uuid, acting_card_instance_id: &Uuid, card_manager: &CardManager, player_manager: &PlayerManager) -> TargetId {

        match self.target_type.as_str() {
            "player" => {
                if let Some(owner) = &self.owner {
                    match owner.as_str() {
                        "self" => {
                            TargetId::Player(*acting_player_id)
                        },
                        "choose" => {
                            // Choose the player to target TODO
                            TargetId::None
                        },
                        "random" => {
                            // Pick a random player to affect TODO
                            TargetId::None
                        }
                        _ => TargetId::None
                    }
                } else {
                    TargetId::None
                }
                
            },
            "card" => { 
                if let Some(owner) = &self.owner {
                    match owner.as_str() {
                        "self" => {
                            if let Some(card) = &self.card {
                                match card.as_str() {
                                    "self" => {
                                        // The target is the current card
                                        TargetId::Card { player_id: *acting_player_id, instance_id: *acting_card_instance_id }
                                    },
                                    _ => TargetId::None,
                                }
                            } else {
                                TargetId::None
                            }
                        },
                        _ => TargetId::None
                    }
                } else {
                    TargetId::None
                }
            },
            _ => TargetId::None
        }

    }


}

