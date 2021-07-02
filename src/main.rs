mod game;
mod gui;

use game::gamestate::Gamestate;
use gui::*;

fn main() {
    print!("SUCK ME!");
    let mut gs = Gamestate::new_setup();
    db_disp(gs.clone_board())
}
