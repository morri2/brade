use super::bb::{Bitboard, BB_CLOSEABLE};
use super::color::*;
use super::position::*;
use super::r#move::*;
pub type Point = usize;

pub struct Gamestate {
    to_play: Color, // YEA I KNOW THIS SHOULD NOT BE A BOOL--- BUT FUCK OFF
    marker_count: [u8; 24],
    bar_count: [u8; 2], // [Player id]
    bb_marker_color: [Bitboard; 2],
}

impl Gamestate {
    pub fn new() -> Self {
        let mut board = [0; 24];
        board[0] = 15;
        board[23] = 15;
        Gamestate {
            to_play: Color::White,
            marker_count: board,
            bar_count: [0, 0],
            bb_marker_color: [
                Bitboard::new(1, Color::White),
                Bitboard::new(1, Color::Black),
            ],
        }
    }

    pub fn marker_count(self) -> [u8; 24] {
        self.marker_count
    }

    pub fn marker_count_at(self, pos: Position, persp: Color) -> u8 {
        match pos {
            Bar => self.bar_count[persp.index()],
            Position::Point(point) => {
                self.marker_count[(point + 12 * persp.index()) % 24] // macro?
            }
            Out => {
                0 // placeholder. Should we have a field for this?
            }
        }
    }

    pub fn apply_move(&mut self, _move: Move) {
        for submove in _move.submoves().iter() {
            self.apply_submove(submove);
        }
    }

    pub fn undo_move(&mut self, _move: Move) {
        for submove in _move.submoves().iter().rev() {
            self.undo_submove(submove);
        }
    }

    fn apply_submove(&mut self, submove: &Submove) {}

    fn undo_submove(&mut self, submove: &Submove) {}
}
