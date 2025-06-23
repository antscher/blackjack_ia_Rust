use std::collections::HashMap;
use crate::game::*;
use rand::Rng;

#[derive(Eq, Hash, PartialEq)]
pub struct State {
    player_cards: Vec<u8>,
    croupier_first_card: u8,
    insurance: bool,
}

impl State {
        pub fn from(game_state :GameState)->State{
        let mut player_cards = Vec::new();
        for card in game_state.get_player_cards().iterator(){
            player_cards.push(card.unwrap().clone());

        }
        player_cards.sort();

        State{
            player_cards : player_cards,
            croupier_first_card : *game_state.get_croupier_first_card().unwrap().unwrap(),
            insurance : game_state.get_insurance()
        }
    }
    
}


pub struct Training {
    pub states: HashMap<State, Vec<f32>>,
}

impl Training {
    pub fn new() -> Self {
        Training {
            states: HashMap::new(),
        }
    }
   
    pub fn add_state(&mut self, state: State) {
        let mut actions = Vec::new();
        if state.croupier_first_card == 1{
            actions = vec![0.0,0.0,0.0,0.0];
        }
        else {
            actions = vec![0.0,0.0,0.0];
        }
        self.states.insert(state, actions);
    }

    pub fn get_best_actions(&self, state: &State) -> Action {
        
    }
}

