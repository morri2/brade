#[derive(Copy, Clone, PartialEq)]
pub enum Position {
    Bar,
    Point(usize),
    Out,
}

impl Position {
    pub fn new(point: usize) -> Self {
        match point {
            0..=23 => Self::Point(point),
            _ => Self::Out,
        }
    }
}
