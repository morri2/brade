use super::color::*;

pub type Bitboard = u32;

pub const BB_CLOSEABLE: Bitboard = 0xDEADBEEF; // PLACEHOLDER
pub const BB_QUADRANT: Bitboard = 0b111111;
pub const BB_FULL: Bitboard = 0b111111_111111_111111_111111;

pub fn flip_persp(bb: Bitboard) -> Bitboard {
    ((bb << 12) | (bb >> 12)) & BB_FULL
}
