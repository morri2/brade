pub const BB_CLOSEABLE: u32 = 0xDEADBEEF;

pub struct BB {
    board: [u32; 2], // from p1 and p2 perspective
}

impl BB {
    pub fn new(board: u32, persp: bool) -> Self {
        let mut bb = BB {board: [0,0]};
        bb.set_board(board, persp);
        bb
    }
    pub fn set_bit(&mut self, bit: usize, value: bool, persp: bool) { // persp: perspective player id
        self.board[persp as usize] &= !(1<< bit);
        self.board[persp as usize] |= (value as u32)<< bit;
        self.board[!persp as usize] &= !(1<< (23-bit));
        self.board[!persp as usize] |= (value as u32)<< (23-bit);
    }
    pub fn bit(self, bit: usize, persp: bool) -> bool {
        self.board[persp as usize] & 1<< bit != 0
    }
    pub fn set_board(&mut self, board: u32, persp: bool) { // Slow!
        for bit in 0..24 {
            self.set_bit(bit, board<< bit != 0, persp)
        }
    }
    pub fn board(self, persp: bool) -> u32{
        return self.board[persp as usize]
    }
    pub fn clear_board(&mut self) {
        self.board = [0,0];
    }
    pub fn enable_board(&mut self) {
        self.board = [(1<<24)-1, (1<<24)-1];
    }
}
