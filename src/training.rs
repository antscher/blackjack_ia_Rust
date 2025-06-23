use core::panic;
use std::collections::HashMap;
use crate::game::*;
use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use serde::Serialize;
use std::io::Write;
use serde_json::Value;

const EPSILON: f32 = 0.2;
const ALPHA : f32 = 0.1;
const GAMMA : f32 = 1.0;

#[derive(Eq, Hash, PartialEq, Clone, Serialize, Debug)]
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

#[derive(Serialize)]
pub struct QTable {
    pub states: HashMap<State, Vec<f32>>,
}

impl QTable {
    pub fn new() -> Self {
        QTable {
            states: HashMap::new(),
        }
    }

    pub fn from(other :QTable) -> QTable{
        QTable {
            states : other.states
        }
    }
   
    pub fn add_state(&mut self, state: State) {
        let actions ;
        if state.croupier_first_card == 1{
            actions = vec![0.0,0.0,0.0,0.0];
        }
        else {
            actions = vec![0.0,0.0,0.0];
        }
        self.states.insert(state, actions);
    }

    pub fn get_best_actions(&mut self, state: &State) -> Action {
        let mut rng = rand::thread_rng();
        let rd: f32 = rng.gen_range(0.0..1.0);
        if !self.states.contains_key(state) {
            self.add_state(state.clone());
        }
        if rd < EPSILON && state.croupier_first_card == 1 {
            let mut rng_action = thread_rng();
            let actions_possible = [Action::Draw, Action::Stand, Action::Double, Action::Insurance];
            *actions_possible.choose(&mut rng_action).unwrap()
        }
        else if rd < EPSILON{
            let mut rng_action = thread_rng();
            let actions_possible = [Action::Draw, Action::Stand, Action::Double, Action::Insurance];
            *actions_possible.choose(&mut rng_action).unwrap()
        }
        else {
            let var = self.states.
            get(state).unwrap()
            .iter()
            .enumerate()
            .filter(|&(_, &x)| !x.is_nan()) // pour éviter les NaN si besoin
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap());

            match var {
                Some((index, _value))=> {
                    match index {
                        0 => Action::Draw,
                        1 => Action::Stand,
                        2 => Action::Double,
                        3 => Action::Insurance,
                        _ => {panic!("Too many arguments in list of action");}
                    }
                }
                None => {panic!("No value found in the list of actions");}
            }
        }
    }

    

    pub fn update(&mut self, state: &State,next_state: &State ,action : &Action, reward : f32){
        let best_state = self.states.
            get(next_state).unwrap()
            .iter()
            .enumerate()
            .filter(|&(_, &x)| !x.is_nan()) // pour éviter les NaN si besoin
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap();


        let td_target = reward + GAMMA * best_state.1;
        let td_delta = td_target - self.states.get(state).expect("Value should be present").get(action.into_index()).unwrap();
        
       let action_index = action.into_index();
        let vec = self.states.get_mut(state).expect("State should exist");
        vec[action_index] = ALPHA * td_delta + vec[action_index];
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let mut json_str = String::from("");
        for state in self.states.iter(){
            json_str.push_str(format!("{:?}",state).as_str());
        }
        let json = serde_json::to_string_pretty(&json_str).unwrap();
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

