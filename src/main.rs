mod game;
mod gui;

use game::*;
use gui::*;

fn main() {
    let mut bb = Gamestate::new();
    gui::db_disp(bb.marker_cnt);
}
