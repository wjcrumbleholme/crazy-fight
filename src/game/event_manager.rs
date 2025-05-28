use super::{card::Card, game_state::{self, GameState}, player::{self, PlayerManager}};



pub enum Event {
    CardPlayed(CardEvent),
    RoundStart(RoundEvent),
}

pub struct CardEvent {
    pub card: Card,
    pub player_id: usize,
}

impl CardEvent {
    pub fn get_card(&self) -> &Card {
        &self.card
    }
    pub fn get_player_id(&self) -> &usize {
        &self.player_id
    }
}

pub struct RoundEvent {
    pub round_number: usize,
}


pub struct EventManager {

}

impl EventManager {
    pub fn new() -> Self {
        Self {

        }
    }
    
    pub fn handle_event(&mut self, event: Event, player_manager: &PlayerManager, game_state: &GameState) {
        match event {
            Event::CardPlayed(ce) => {
                //Card played, run its on_play if it has one
                if let Some(ability) = ce.get_card().get_ability() {
                    if ability.get_trigger() == Some("on_play".to_string()) {
                        let player = player_manager.get_player_by_id(*ce.get_player_id()).unwrap(); 

                        if ability.conditions_met(player) {
                            ability.apply_effects()
                        }
                    }
                }
            },
            Event::RoundStart(re) => {
                //New round started, loop through all cards in play and trigger effects
            }
        }
    }
}