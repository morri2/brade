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
    bb_singleton: Bitboard,
}

impl Gamestate {
    pub fn new() -> Self {
        let mut board = [0; 24];
        board[0] = 15;
        board[12] = 15;
        Gamestate {
            to_play: Color::White,
            marker_count: board,
            bar_count: [0, 0],
            bb_marker_color: [
                Bitboard::new(1, Color::White),
                Bitboard::new(1, Color::Black),
            ],
            bb_singleton: Bitboard::new(0, Color::White),
        }
    }

    pub fn marker_count(&self) -> [u8; 24] {
        self.marker_count
    }

    pub fn marker_count_at(&self, pos: Position, persp: Color) -> u8 {
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

    pub fn bb_is_occupied(&self, persp: Color) -> u32 {
        // might be best used as discrete func later idk
        self.bb_marker_color[Color::Black.index()].board(persp)
            | self.bb_marker_color[Color::White.index()].board(persp)
    }

    pub fn bb_is_color(&self, color: Color, persp: Color) -> u32 {
        self.bb_marker_color[color.index()].board(persp)
    }

    pub fn bb_legal_landing_spaces(&self, persp: Color) -> u32 {
        let mut legal_landing = !self.bb_is_occupied(persp) // all empty points are legal
        | (self.bb_is_color(persp, persp) & BB_CLOSEABLE) // closable points
        | (self.bb_is_color(persp.reverse(), persp) & self.bb_singleton.board(persp)); // capturable points

        if self.bar_count[persp.index()] != 0 {
            // get out from bar
            legal_landing &= 0x3F;
            if legal_landing == 0 {
                legal_landing = self.bb_is_color(persp.reverse(), persp);
            }
        } else { // THIS IS SUBOPTIMAL AND UNREADABLE FIX LATER LMAO
            let mut i = 0;
            let opp_closed = self.bb_is_color(persp.reverse(), persp);
            // add spränga condition!
            let mut sprangable: u32 = 0;
            while i < 24 { // we can exclued the last one
                if (opp_closed << i) & 0x3f == 0x3f {
                    sprangable |= 0x3f << i;
                    i += 1;
                } else {
                    i += u32::max(u32::trailing_zeros(opp_closed << i),1);
                }
            }
            legal_landing |= sprangable
        }
        legal_landing
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
