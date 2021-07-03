mod game;
mod gui;

use game::gamestate::Gamestate;
use gui::*;
use game::r#move::*;
use game::position::*;

fn main() {
    println!("SUCK ME!");
    let mut gs = Gamestate::new_setup(); 
    db_print(gs.clone_board());
    gs.apply_submove(Submove::new(Position::new(0), Position::new(2), 0));
    db_print(gs.clone_board());
    gs.apply_submove(Submove::new(Position::new(0), Position::new(12), 0));
    db_print(gs.clone_board());
    gs.flip();
    db_print(gs.clone_board());
}

