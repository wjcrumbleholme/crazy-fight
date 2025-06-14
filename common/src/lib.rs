use std::{collections::HashMap, fs::{self, read_dir}};

use game::card::{CardManager, Card};
use game::deck::DeckManager;
use game::event_manager::{DrawSelector, Event, EventManager};
use game::game_state::GameState;
use game::player::PlayerManager;
use uuid::Uuid;


pub mod server;
pub mod game;



pub struct GameManger {
    deck_manager: DeckManager,
    card_manager: CardManager,
    game_state: GameState,
    player_manager: PlayerManager,
    event_manager: EventManager,
}

impl GameManger {
    pub fn new() -> Self {
        Self {
            deck_manager: DeckManager::new(),
            card_manager: CardManager::new(),
            game_state: GameState::new(),
            player_manager: PlayerManager::new(),
            event_manager: EventManager::new(),
        }
    }

    pub fn get_deck_manager(&self) -> &DeckManager {
        &self.deck_manager
    }

    pub fn get_card_manager(&self) -> &CardManager {
        &self.card_manager
    }

    pub fn get_card_manager_mut(&mut self) -> &mut CardManager {
        &mut self.card_manager
    }

    pub fn get_deck_manager_mut(&mut self) -> &mut DeckManager {
        &mut self.deck_manager
    }

    pub fn load_deck(&mut self, deck_path: &str) {
        // Pass all cards in and register them with the card manager
        

        // Load in the cards - the deck info will be in pack_info.json
        let paths = read_dir(deck_path).unwrap();

        for path in paths {
            if path.as_ref().unwrap().path().extension().unwrap() == "json" {
                if !(path.as_ref().unwrap().path().file_name().unwrap() == "deck_info.json") {
                    //Register this card with the card Manager
                    self.card_manager.register_card(path.as_ref().unwrap().path().to_str().unwrap());
                }
                
            }
        }

        // Open the deck_info.json then populate the piles as specified
        let data = fs::read_to_string(deck_path.to_owned() + "/deck_info.json").expect("Could not read deck_info.json");
        let deck_info: HashMap<String, usize> = serde_json::from_str(&data).expect("JSON was not well-formatted");

        for (card_id, count) in deck_info {
            for _ in 0..count {
                if let Some(card) = self.card_manager.get_card(&card_id) {
                    match card {
                        Card::Character(_c) => {
                            //Add to character draw pile
                            self.deck_manager.add_character_draw_pile(card_id.clone());
                        },
                        Card::SuperCharacter(c) => {
                            self.deck_manager.add_super_character_draw_pile(card_id.clone());
                        }
                        _ => {
                            //Add to regular draw pile
                            self.deck_manager.add_item_draw_pile(card_id.clone());
                        }
                    }
                    println!("Card ID {} succesfully added to the draw pile!", card_id)
                } else {
                    println!("Card ID {} not found in registry", card_id)
                }
            }
        }
    }


    pub fn test_create_player(&mut self) -> Uuid {
        let test_player_id = self.player_manager.create_player("test".to_owned());
        test_player_id
    }


    pub fn test_draw_pile(&mut self, player_id: Uuid) -> Option<Vec<Uuid>> {
        
        //Add all of the current cards in the players hand to the discard pile and delete them from the card_manager
        if let Some(player) = self.player_manager.get_player_by_id(&player_id) {
            for instance_id in player.get_hand().to_vec() {
                if let Some(card) = self.card_manager.get_card_from_instance_id(&instance_id) {
                    // Add card to discard pile
                    self.deck_manager.add_discard_pile(card.get_card_id().to_owned());
                }
                self.card_manager.deinstansiate_card(&instance_id);
            }
        } 

        //Clear the players hand
        
        self.event_manager.handle_event(Event::DrawCard { player_id: player_id, pile: "standard".to_owned(), selector: DrawSelector::Random }, &mut self.player_manager, &self.game_state, &mut self.card_manager, &mut self.deck_manager);
        self.event_manager.handle_event(Event::DrawCard { player_id: player_id, pile: "standard".to_owned(), selector: DrawSelector::Random }, &mut self.player_manager, &self.game_state, &mut self.card_manager, &mut self.deck_manager);
        self.event_manager.handle_event(Event::DrawCard { player_id: player_id, pile: "standard".to_owned(), selector: DrawSelector::Random }, &mut self.player_manager, &self.game_state, &mut self.card_manager, &mut self.deck_manager);
        self.event_manager.handle_event(Event::DrawCard { player_id: player_id, pile: "standard".to_owned(), selector: DrawSelector::Random }, &mut self.player_manager, &self.game_state, &mut self.card_manager, &mut self.deck_manager);
        self.event_manager.handle_event(Event::DrawCard { player_id: player_id, pile: "standard".to_owned(), selector: DrawSelector::Random }, &mut self.player_manager, &self.game_state, &mut self.card_manager, &mut self.deck_manager);
        self.event_manager.handle_event(Event::DrawCard { player_id: player_id, pile: "standard".to_owned(), selector: DrawSelector::Random }, &mut self.player_manager, &self.game_state, &mut self.card_manager, &mut self.deck_manager);

        if let Some(player) = self.player_manager.get_player_by_id(&player_id) {
            return Some(player.get_hand().to_vec())
        } 
        None
    }
}