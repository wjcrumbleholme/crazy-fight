use macroquad::{prelude::rand};
use uuid::Uuid;

use super::{card::CardManager, deck::{self, DeckManager}, game_state::GameState, player::{self, PlayerManager}};



pub enum Event {
    CardPlayed(CardEvent),
    RoundStart(RoundEvent),
    DrawCard { player_id: Uuid, pile: String, selector: DrawSelector }
}

pub enum DrawSelector {
    Random,
    CardId(String),
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
    
    pub fn handle_event(&mut self, event: Event, player_manager: &mut PlayerManager, game_state: &GameState, card_manager: &mut CardManager, deck_manager: &mut DeckManager) {
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
                match pile.as_str() {
                    "standard" => {
                        match selector {
                            DrawSelector::Random => {
                                // Draw a random card from the standard draw pile
                                if let Some(card_id) = deck_manager.get_random_card_and_remove_item_pile() {
                                    if let Some(mut card_to_inst) = card_manager.get_card_from_id_clone(&card_id) {
                                        let instance_id = Uuid::new_v4();
                                        card_to_inst.set_instance_id(instance_id);
                                        // Add the card to the instansitated card hashmap
                                        card_manager.instansiate_card(&instance_id, card_to_inst);
                                        // Add the intance card id to the players hand
                                        if let Some(player) = player_manager.get_player_by_id_mut(&player_id) {
                                            player.add_card_instance_id_to_hand(instance_id);
                                        }
                                    } 
                                } else {
                                    // None was returned, standard pile must be empty 
                                }
                                

                            }
                            _ => ()
                        }
                    },
                    "character" => {
                        match selector {
                            DrawSelector::Random => {
                                // Pick a random character from the character pile
                            },
                            DrawSelector::CardId(cid) => {
                                // Pick a character from the character pile (people picker)
                            },
                            _ => ()
                        }
                    },
                    "super_character" => {
                        match selector {
                            DrawSelector::CardId(cid) => {
                                // Pick a super card - will have to handle super card picker later
                            },
                            _ => ()
                        }
                    }
                    _ => ()
                }
            },
            _ => ()
        }
    }
}