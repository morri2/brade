use super::color::*;

pub const BB_CLOSEABLE: u32 = 0xDEADBEEF;
pub struct Bitboard {
    board: u32,
}

impl Bitboard {
    pub fn new(board: u32, persp: Color) -> Self {
        let mut bb = Bitboard { board: 0 };
        bb.set_board(board, persp);
        bb
    }

    fn flip_board(board: u32) -> u32 {
        (board << 12) | board
    }

    pub fn board(&self, persp: Color) -> u32 {
        (self.board << (12 * persp.index())) | self.board
    }

    pub fn set_board(&mut self, board: u32, persp: Color) {
        self.board = (board << (12 * persp.index())) | board
    }

    pub fn set_bit(&mut self, bit: usize, value: bool, persp: Color) {
        // persp: perspective player id
        let board = (self.board(persp) & !(1 << bit)) | (value as u32) << bit;
        self.set_board(board, persp)
    }

    pub fn bit(&self, bit: usize, persp: Color) -> bool {
        self.board(persp) & 1 << bit != 0
    }

    pub fn clear_board(&mut self) {
        self.board = 0
    }
}
