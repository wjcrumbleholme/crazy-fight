use std::collections::HashMap;

use super::card::Card;


pub struct PlayerManager {
    players: HashMap<usize, Player>,
    next_id: usize
}

pub struct Player {
    id: usize,
    name: String,
    hand: Vec<Card>,
    table: Vec<Card>,
}


impl Player {
    pub fn get_hand(&self) -> Vec<Card> {
        self.hand.clone()
    }
}

impl PlayerManager {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn create_player(&mut self, name: String) {
        let id = self.next_id;
        self.next_id += 1;
        let new_player = Player {
            id: id,
            name: name,
            hand: vec![],
            table: vec![]
        };
        self.players.insert(id, new_player);
    }

    pub fn get_player_by_id(&self, id: usize) -> Option<&Player> {
        self.players.get(&id)
    }

    pub fn get_player_by_id_mut(&mut self, id: usize) -> Option<&mut Player> {
        self.players.get_mut(&id)
    }

    pub fn remove_player(&mut self, id: usize) {
        self.players.remove(&id);
    }

}