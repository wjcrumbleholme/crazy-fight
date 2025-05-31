use std::collections::HashMap;

use uuid::Uuid;

use super::card::status::Status;



pub struct PlayerManager {
    players: HashMap<Uuid, Player>,
    next_id: usize
}

pub struct Player {
    id: Uuid,
    name: String,
    hand: Vec<Uuid>,
    table: Vec<Uuid>,
    status_effects: Vec<Status>,
}


impl Player {
    pub fn get_hand(&self) -> &Vec<Uuid> {
        &self.hand
    }
    
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn add_status_effect(&mut self, effect: Status) {
        &self.status_effects.push(effect);
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
        let id = Uuid::new_v4();
        let new_player = Player {
            id: id,
            name: name,
            hand: vec![],
            table: vec![],
            status_effects: vec![]
        };
        self.players.insert(id, new_player);
    }

    pub fn get_player_by_id(&self, id: &Uuid) -> Option<&Player> {
        self.players.get(id)
    }


    pub fn get_player_by_id_mut(&mut self, id: &Uuid) -> Option<&mut Player> {
        self.players.get_mut(id)
    }

    pub fn remove_player(&mut self, id: &Uuid) {
        self.players.remove(id);
    }


}