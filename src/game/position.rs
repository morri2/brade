#[derive(Copy,Clone)]
pub enum Position {
    Bar,
    Point(usize),
    Out,
}

impl Position {}
