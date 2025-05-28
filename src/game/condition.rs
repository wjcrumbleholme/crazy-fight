use serde::Deserialize;

use super::player::{self, Player};

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
    pub fn is_met(&self, player: &Player) -> bool {
        match self.condition.as_deref() {
            Some("num_cards_in_hand") => {
                let left = player.get_hand().len() as i32;
                let right = self.int.unwrap_or(0);
                compare_ints(&self.operator.clone().unwrap_or("=".to_owned()), left, right)
            }
            _ => false
        }
    }
}