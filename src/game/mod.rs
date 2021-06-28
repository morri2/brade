// TODO!
// Gamestate
// - BBs when possible
// Perspective gets
// - 2 BBs (perspective)
// Movedata reversable (sub moves?)
// - hold capture amt
// Find leagal moves (tree?)
mod bb::{BB,BB_CLOSEABLE};
type point = usize;
const P1_PERSP = false;
const P2_PERSP = true;



struct Gamestate {
    marker_cnt: u8[24],
    bar_cnt: u8[2],
    bb_color: BB[color],
    bb_singleton: BB,
}

impl Gamestate {
    fn new() -> Self {
        board = [0; 24];
        board[0] = 15;
        board[23] = 15;
        Gamestate {
            marker_cnt: board,
            bar_cnt: [0,0],
            bb_color: [BB::new(1,P1_PERSP), BB::new(1,P2_PERSP)],
            bb_singleton: BB::new(0,P1_PERSP),
        }
    }
}


