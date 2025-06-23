pub mod card;
pub mod game;
pub mod training;
use game::*;
use training::*;

use crate::training::QTable;

fn main() {
    let mut game_state = GameState::new();
    let results = game(&mut game_state, 10.0);
    println!("Fin de la partie. Results : {}", results);
    //println!("Nouvelle partie");
    let state = State::from(game_state);
    let mut Q = QTable::new(); 
    Q.add_state(state);
    let res = Q.save("qtable.json");
    match res {
        Ok(_)=>{print!("success");},
        Err(e) => {print!("failed, {:?}", e);}
    }


}
