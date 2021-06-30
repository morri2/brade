mod game;
mod gui;

use game::gamestate::Gamestate;
use gui::*;

fn main() {
    let mut bb = Gamestate::new();
    gui::db_disp(bb.marker_count());
}
