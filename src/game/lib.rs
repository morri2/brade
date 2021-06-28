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



struct Gamestate {
    marker_cnt: u8[24]
    bb_color: BB[2,2] // [perspective, color]
}
