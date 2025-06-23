pub mod card;
pub mod game;
use game::*;
pub mod training;

fn main() {
    loop {
        let state = GameState::new();
        let results = game(state, 10.0);
        println!("Fin de la partie. Results : {}", results);
        println!("Nouvelle partie");
    }


}
