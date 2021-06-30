#[derive(Copy, Clone)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn reverse(self) -> Color {
        match self {
            Color::Black => return Color::White,
            Color::White => return Color::Black,
        }
    }

    pub fn index(self) -> usize {
        match self {
            Color::Black => return 0,
            Color::White => return 1,
        }
    }
}
