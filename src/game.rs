use crate::card::*;
use std::io;
use std::io::Write;

/*
    Actions possibles du joueur au Blackjack :
    1. Tirer une carte (Hit)
    2. Rester (Stand)
    3. Doubler la mise (Double Down)
    4. Prendre une assurance (Insurance)
    //5. Séparer (Split) to implement later

*/

#[derive(Clone, Debug, Copy, Eq, Hash, PartialEq)]
pub enum Action {
    Draw,
    Stand,
    Double,
    Insurance,
}
impl Action {
    pub fn into_index(&self) -> usize {
        match self {
            Action::Draw => 0,
            Action::Stand => 1,
            Action::Double => 2,
            Action::Insurance => 3,
        }
    }
}

#[derive(Clone)]
pub struct GameState {
    pub continue_game: bool,         // if the game is still ongoing
    pub player_cards: PackOfCards,   // cards of the player
    pub croupier_cards: PackOfCards, // cards of the dealer
    pub packet: PackOfCards,         // packet of cards that have not been played
    pub discard: PackOfCards,        // packet of cards that have been played
    pub insurance: bool,             // if the player has taken insurance
    pub double: bool,                // if the player has doubled down
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            continue_game: true,
            player_cards: PackOfCards::new(),
            croupier_cards: PackOfCards::new(),
            packet: PackOfCards::init(),
            discard: PackOfCards::new(),
            insurance: false,
            double: false,
        }
    }

    pub fn from(other: &GameState) -> GameState {
        GameState {
            continue_game: other.continue_game.clone(),
            player_cards: other.player_cards.clone(),
            croupier_cards: other.croupier_cards.clone(),
            packet: other.packet.clone(),
            discard: other.discard.clone(),
            insurance: other.insurance.clone(),
            double: other.double.clone(),
        }
    }

    pub fn get_player_cards(&self) -> &PackOfCards {
        &self.player_cards
    }

    pub fn as_mut_ref(&mut self) -> &mut Self {
        self
    }

    pub fn get_croupier_first_card(&self) -> Option<&Card> {
        self.croupier_cards.get_card(0)
    }

    pub fn get_insurance(&self) -> bool {
        self.insurance
    }

    pub fn play(&mut self, action: Action) -> Result<GameState, &str> {
        let mut new_state = GameState::from(self);
        match action {
            Action::Draw => {
                let card = new_state.packet.pick().unwrap();
                // dbg!(" you draw : {} ", &card);
                new_state.player_cards.add_card(card.clone());
                new_state.discard.add_card(card.clone());
            }
            Action::Stand => {
                new_state.continue_game = false;
                // dbg!("You stand");
            }
            Action::Double => {
                let card = new_state.packet.pick().unwrap();
                // dbg!(" you double and draw : {} ", &card);
                new_state.player_cards.add_card(card.clone());
                new_state.discard.add_card(card.clone());
                new_state.continue_game = false;
                new_state.double = true;
            }
            Action::Insurance => {
                if (new_state.croupier_cards.get_card(0).unwrap().unwrap() == &1)
                    && !new_state.insurance
                {
                    new_state.insurance = true;
                    // dbg!("You take insurance");
                } else {
                    return Err("Impossible to take an insurance");
                }
            }
        }
        if new_state.player_cards.sum() >= 21 {
            new_state.continue_game = false;
        }
        Ok(new_state)
    }

    pub fn results(&self, bet: f32) -> f32 {
        let mut total: f32 = 0.0;

        match (self.insurance, self.croupier_cards.sum()) {
            (true, 21) => total += bet / 2.0,
            (true, _) => total -= bet / 2.0,
            _ => {}
        }
        match (
            self.croupier_cards.sum(),
            self.player_cards.sum(),
            self.double,
        ) {
            (_, 21, true) => {
                total += bet * 3.0;
            }
            (_, 21, false) => {
                total += bet * 1.5;
            }
            (_, player_point, true) if player_point > 21 => {
                total -= bet * 2.0;
            }
            (_, player_point, false) if player_point > 21 => {
                total -= bet;
            }
            (croupier_point, _, true) if croupier_point > 21 => {
                total += bet * 2.0;
            }
            (croupier_point, _, false) if croupier_point > 21 => {
                total += bet;
            }
            (croupier_point, player_point, true) if croupier_point > player_point => {
                total -= bet * 2.0;
            }
            (croupier_point, player_point, true) if croupier_point < player_point => {
                total += bet * 2.0;
            }
            (croupier_point, player_point, false) if croupier_point > player_point => {
                total -= bet;
            }
            (croupier_point, player_point, false) if croupier_point < player_point => {
                total += bet;
            }
            _ => {}
        }

        total
    }
}

pub fn game(game_state: &mut GameState, bet: f32) -> f32 {
    let card = game_state.packet.pick().unwrap();
    println!(" you draw : {} ", &card);
    game_state.player_cards.add_card(card.clone());
    game_state.discard.add_card(card.clone());

    let card = game_state.packet.pick().unwrap();
    println!(" you draw : {} ", &card);
    game_state.player_cards.add_card(card.clone());
    game_state.discard.add_card(card.clone());

    let card = game_state.packet.pick().unwrap();
    println!(" croupier draw : {} ", &card);
    game_state.croupier_cards.add_card(card.clone());
    game_state.discard.add_card(card.clone());

    while game_state.continue_game {
        println!("Choisissez une action : draw, stand, double, insurance");
        print!("> ");
        io::stdout().flush().unwrap(); // Pour s'assurer que l'invite est bien affichée

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        let action = match input.as_str() {
            "draw" => Action::Draw,
            "stand" => Action::Stand,
            "double" => Action::Double,
            "insurance" => Action::Insurance,
            _ => {
                println!("Action invalide. Essayez encore.");
                continue;
            }
        };

        match game_state.play(action) {
            Ok(new_state) => {
                *game_state = new_state;
                println!(
                    "État mis à jour. Somme des cartes du joueur : {}",
                    game_state.player_cards.sum()
                );
                println!("Points du croupier : {} ", game_state.croupier_cards.sum());
            }
            Err(e) => {
                println!("Erreur : {}", e);
            }
        }
    }

    println!(
        "Fin de la partie. Somme finale des cartes joueur : {}",
        game_state.player_cards.sum()
    );

    while game_state.croupier_cards.sum() < 17 {
        let card = game_state.packet.pick().unwrap();
        println!(" Croupier drew : {} ", card);
        game_state.croupier_cards.add_card(card.clone());
        game_state.discard.add_card(card.clone());
    }

    game_state.results(bet)
}
