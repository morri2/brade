
use super::bb::{BB,BB_CLOSEABLE};
use super::r#move::*;
pub type point = usize;
pub const P1_PERSP: bool = false;
pub const P2_PERSP: bool = true;



pub struct Gamestate {
    to_play: bool, // YEA I KNOW THIS SHOULD NOT BE A BOOL--- BUT FUCK OFF
    marker_count: [u8;24],
    bar_count: [u8; 2], // [Player id]
    bb_marker_color: [BB; 2],
}

impl Gamestate {
    pub fn new() -> Self {
        let mut board = [0; 24];
        board[0] = 15;
        board[23] = 15;
        Gamestate {
            to_play: P1_PERSP,
            marker_count: board,
            bar_count: [0,0],
            bb_marker_color: [BB::new(1,P1_PERSP), BB::new(1,P2_PERSP)],
        }
    }

    pub fn marker_count(self) -> [u8; 24]{
        self.marker_count
    }

    pub fn apply_move(&mut self, _move: Move){
        for submove in _move.submoves().iter(){
            self.apply_submove(submove);
        }
    }

    pub fn undo_move(&mut self, _move: Move){
        for submove in _move.submoves().iter().rev(){
            self.undo_submove(submove);
        }
    }

    fn apply_submove(&mut self, submove: &Submove){

    }

    fn undo_submove(&mut self, submove: &Submove){

    }

}
