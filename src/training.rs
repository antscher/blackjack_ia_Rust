use crate::game::*;
use core::panic;
use dashmap::DashMap;
use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

const ALPHA: f32 = 0.1;
const GAMMA: f32 = 0.5;

#[derive(Eq, Hash, PartialEq, Clone, Serialize, Debug)]
pub struct State {
    player_cards: Vec<u8>,
    croupier_first_card: u8,
    insurance: bool,
}

impl State {
    pub fn from(game_state: &GameState) -> State {
        let mut player_cards = Vec::new();
        for card in game_state.get_player_cards().iterator() {
            player_cards.push(card.unwrap().clone());
        }
        player_cards.sort();

        State {
            player_cards: player_cards,
            croupier_first_card: *game_state.get_croupier_first_card().unwrap().unwrap(),
            insurance: game_state.get_insurance(),
        }
    }
}

#[derive(Clone)]
pub struct QTable {
    pub states: DashMap<State, Vec<f32>>,
}

impl QTable {
    pub fn new() -> Self {
        QTable {
            states: DashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn from(other: QTable) -> QTable {
        QTable {
            states: other.states,
        }
    }

    pub fn add_state(&mut self, state: State) {
        if self.states.contains_key(&state) {
            return;
        }
        let actions;

        if state.croupier_first_card == 1 && state.player_cards.len() == 2 && !state.insurance {
            actions = vec![0.0, 0.0, 0.0, 0.0];
        } else {
            actions = vec![0.0, 0.0, 0.0];
        }

        self.states.insert(state, actions);
    }

    pub fn get_best_action(&mut self, state: &State, epsilon: f32) -> Action {
        let mut rng = rand::thread_rng();
        let rd: f32 = rng.gen_range(0.0..1.0);
        if !self.states.contains_key(state) {
            self.add_state(state.clone());
        }
        if rd < epsilon
            && state.croupier_first_card == 1
            && state.player_cards.len() == 2
            && !state.insurance
        {
            let mut rng_action = thread_rng();
            let actions_possible = [
                Action::Draw,
                Action::Stand,
                Action::Double,
                Action::Insurance,
            ];
            *actions_possible.choose(&mut rng_action).unwrap()
        } else if rd < epsilon {
            let mut rng_action = thread_rng();
            let actions_possible = [Action::Draw, Action::Stand, Action::Double];
            *actions_possible.choose(&mut rng_action).unwrap()
        } else {
            let vec = self.states.get(state).expect("State should be present");
            let var = vec
                .iter()
                .enumerate()
                .filter(|&(_, &x)| !x.is_nan()) // pour éviter les NaN si besoin
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap());

            let action;
            match var {
                Some((index, _value)) => match index {
                    0 => {
                        action = Action::Draw;
                    }
                    1 => {
                        action = Action::Stand;
                    }
                    2 => {
                        action = Action::Double;
                    }
                    3 => {
                        action = Action::Insurance;
                    }
                    _ => {
                        panic!("Too many arguments in list of action");
                    }
                },
                None => {
                    panic!("No value found in the list of actions");
                }
            }
            drop(vec);
            action
        }
    }

    pub fn update(
        &mut self,
        state: &State,
        next_state: &State,
        action: &Action,
        next_action: &Option<Action>,
        reward: f32,
    ) {
        if reward.is_nan() {
            panic!("Reward is NaN, check your game logic");
        } else if reward.is_infinite() {
            panic!("Reward is infinite, check your game logic");
        } else if reward == 0.0 {
            return;
        }

        let td_target: f32;

        match next_action {
            Some(_) => {
                let next_vec = self
                    .states
                    .get(next_state)
                    .expect("Next state should be present");

                let best_action = next_vec
                    .iter()
                    .enumerate()
                    .filter(|&(_, &x)| !x.is_nan()) // pour éviter les NaN si besoin
                    .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                    .unwrap();

                td_target = reward + GAMMA * best_action.1;

                drop(next_vec);
            }
            None => {
                td_target = reward;
            }
        }

        if let Some(mut curr_vec) = self.states.get_mut(state) {
            let current_q = curr_vec[action.into_index()];

            let td_delta = td_target - current_q;

            curr_vec[action.into_index()] += ALPHA * td_delta;

            drop(curr_vec);
        } else {
            panic!("State {:?} not found in QTable", state);
        }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let mut map: HashMap<_, _> = HashMap::new();

        for entry in self.states.iter() {
            map.insert(entry.key().clone(), entry.value().clone());
        }

        let mut json_str = String::from("");
        for (state, action) in map.iter() {
            json_str.push_str(format!("{:?} : {:?}", state, action).as_str());
            json_str.push('\n');
        }
        let json = serde_json::to_string_pretty(&json_str).unwrap();
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn trainnig_q(&mut self, game_state: &mut GameState, bet: f32, epsilon: f32) -> f32 {
        let card = game_state.packet.pick().unwrap();
        game_state.player_cards.add_card(card.clone());
        game_state.discard.add_card(card.clone());

        let card = game_state.packet.pick().unwrap();
        game_state.player_cards.add_card(card.clone());
        game_state.discard.add_card(card.clone());

        let card = game_state.packet.pick().unwrap();
        game_state.croupier_cards.add_card(card.clone());
        game_state.discard.add_card(card.clone());

        let mut map: HashMap<usize, (Option<Action>, Option<State>)> = HashMap::new();
        let mut i = 0;

        while game_state.continue_game {
            let state = State::from(game_state);
            let action = self.get_best_action(&state, epsilon);

            map.insert(i, (Some(action), Some(state)));
            i += 1;

            match game_state.play(action) {
                Ok(new_game_state) => {
                    *game_state = new_game_state;
                }
                Err(e) => {
                    println!("Erreur : {}", e);
                }
            }
        }
        let state = State::from(game_state);
        self.add_state(state.clone());

        map.insert(i, (None, Some(state)));

        while game_state.croupier_cards.sum() < 17 {
            let card = game_state.packet.pick().unwrap();
            game_state.croupier_cards.add_card(card.clone());
            game_state.discard.add_card(card.clone());
        }

        let reward = game_state.results(bet);

        for (index, (action, state)) in &map {
            match (state, action) {
                (Some(state), Some(action)) if *index < map.len() - 1 => self.update(
                    &state,
                    &map.get(&(index + 1)).unwrap().1.clone().unwrap(),
                    &action,
                    &map.get(&(index + 1)).unwrap().0.clone(),
                    reward,
                ),
                _ => {}
            }
        }
        reward
    }
}
