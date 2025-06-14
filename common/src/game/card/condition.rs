use serde::Deserialize;
use uuid::Uuid;

use super::super::player::PlayerManager;

use super::{CardManager};

// Condition
#[derive(Deserialize, Debug, Clone)]
pub struct Condition {
    condition: Option<String>,
    operator: Option<String>,
    string: Option<String>,
    bool: Option<bool>,
    int: Option<i32>,
}


pub fn compare_ints(operator: &str, left: i32, right: i32) -> bool {
    match operator {
        "equal" => left == right,
        "not_equal" => left != right,
        "greater_than" => left > right,
        "less_than" => left < right,
        "greater_than_or_equal_to" => left >= right,
        "less_than_or_equal_to" => left <= right,
        _ => false,
    }
}


impl Condition {
    pub fn is_met(&self, player_id: &Uuid, card_instance_id: &Uuid, card_manager: &CardManager, player_manager: &PlayerManager) -> bool {
        match self.condition.as_deref() {
            Some("num_cards_in_hand") => {
                if let Some(player) = player_manager.get_player_by_id(&player_id) {
                    let left = player.get_hand().len() as i32;
                    let right = self.int.unwrap_or(0);
                    compare_ints(&self.operator.clone().unwrap_or("equal".to_owned()), left, right)
                } else {
                    false
                }
                
            }
            _ => false
        }
    }
}