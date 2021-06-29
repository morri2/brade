
pub enum Color{
    Black,
    White,
}

impl Color{
    pub fn reverse(self) -> Color{
        match self{
            Color::Black => return Color::White,
            Color::White => return Color::Black,
        }
    }
}
