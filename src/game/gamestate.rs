use super::bitboard::*;
use super::board::*;
use super::color::*;
use super::position::*;
use super::r#move::*;

pub struct Gamestate {
    board: Board,
}

impl Gamestate {
    pub fn new(board: Board) -> Self {
        Gamestate { board }
    }

    pub fn new_setup() -> Self {
        Self::new(Board::new_setup())
    }

    pub fn clone_board(&self) -> Board {
        self.board.clone()
    }
    /*

        pub fn marker_color_at(&self, pos: Position, persp: Color) -> Option<Color>{
            if let Position::Point(point) = pos {
                if self.bb_marker_color[Color::White.index()].board(persp) << point != 0{
                    return Some(Color::White)
                } else if self.bb_marker_color[Color::Black.index()].board(persp) << point != 0 {
                    return Some(Color::Black)
                }
            }
            None
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

    <    pub fn bb_is_color(&self, color: Color, persp: Color) -> u32 {
            self.bb_marker_color[color.index()].board(persp)
        }

        pub fn bb_generate_legal_dests(&self, persp: Color) -> u32 {
            let mut legal_landing = !self.bb_is_occupied(persp) // all empty points are legal
            | (self.bb_is_color(persp, persp) & BB_CLOSEABLE) // closable points
            | (self.bb_is_color(persp.reverse(), persp) & self.bb_singleton.board(persp)); // capturable points

            if self.bar_count[persp.index()] != 0 {
                let legal_dests = !self.bb_is_occupied(persp) // all empty points are legal
                | (self.bb_is_color(persp.reverse(), persp) & self.bb_singleton.board(persp)) & 0b111111;
                // get out from bar
                if legal_dests != 0 {
                    legal_dests
                } else {
                    self.bb_is_color(persp.reverse(), persp) & 0b111111
                }
            } else {
                0 // Implement proper spräng for moving off the bar
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
        */
}
