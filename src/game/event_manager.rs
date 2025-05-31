use uuid::Uuid;

use super::{card::CardManager, game_state::GameState, player::PlayerManager};



pub enum Event {
    CardPlayed(CardEvent),
    RoundStart(RoundEvent),
    DrawCard { player_id: Uuid, pile: String, selector: DrawSelector }
}

pub enum DrawSelector {
    Random,
    CardId(String)
}

pub struct CardEvent {
    pub card_instance_id: Uuid,
    pub player_id: Uuid,
}

impl CardEvent {
    pub fn card_instance_id(&self) -> &Uuid {
        &self.card_instance_id
    }
    pub fn player_id(&self) -> &Uuid {
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
    
    pub fn handle_event(&mut self, event: Event, player_manager: &mut PlayerManager, game_state: &GameState, card_manager: &mut CardManager) {
        match event {
            Event::CardPlayed(ce) => {

                //Need to do everything before mutating in process effects
                let mut to_process = Vec::new();

                //Card played, run its on_play if it has one
                //Get card hope it exisits
                if let Some(card) = card_manager.get_card_from_instance_id(ce.card_instance_id()) {
                    //Get the cards ability if it has one
                    if let Some(ability) = card.get_ability() {
                        //Check the trigger and condtions
                        if ability.get_trigger() == "on_play" && ability.conditions_met(ce.player_id(), ce.card_instance_id(), card_manager, player_manager){
                            to_process.push((ce.player_id.clone(), ce.card_instance_id.clone(), ability.clone()));
                        }
                    }
                }

                //Process effects now all of the immutable stuff is done
                for (player_id, card_instance_id, ability) in to_process {
                    ability.process_effects(&player_id, &card_instance_id, player_manager, card_manager);
                }
                
            },
            Event::RoundStart(re) => {
                // First loop through every card / player and any effects with duration, decrement them
                // Then check each card in play to see if it has an on_round_start trigger
            },
            Event::DrawCard { player_id, pile, selector } => {
                // When card is drawn, instansiate it (give it an instance id and assign it to a player)
            },
            _ => ()
        }
    }
}