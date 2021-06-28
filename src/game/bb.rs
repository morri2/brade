const BB_CLOSEABLE: BB = 0xDEADBEEF;

struct BB {
    board: u32[2] // from p1 and p2 perspective
}

impl BB {
    fn set_bit(self, bit: usize, value: bool, persp: bool) { // persp: perspective player id
        self.board[persp as usize] &= ~(1<< bit);
        self.board[persp as usize] |= value<< bit;
        self.board[persp as usize] &= ~(1<< bit);
        self.board[persp as usize] |= value<< (23-bit);
    }
    fn get_bit(self, bit: usize, persp: bool) -> bool {
        (self.board[persp as usize] & 1<< bit) as bool;
    }
    fn set_board(self, board: u32, persp: bool) { // Slow!
        for bit in 0..32 {
            self.set_bit(bit, (board<< bit), persp)
        }
    }
    fn get_board(self, persp: bool) -> u32{
        return self.board[persp as usize]
    }
    fn clear_board(self) {
        self.board = {0,0};
    }
    fn enable_board(self) {
        self.board = {(1<<24) -1, (1<<24) -1};
    }
}
