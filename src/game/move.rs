use super::position::Position;

pub struct Move {
    submoves: Vec<Submove>,
}

impl Move {
    pub fn submoves(self) -> Vec<Submove> {
        self.submoves
    }
}

pub struct Submove {
    src: Position,
    dest: Position,
    captured: u8,
}

impl Submove {
    pub fn new(src: Position, dest: Position, captured: u8) -> Self {
        Self {
            src,
            dest,
            captured,
        }
    }

    pub fn src(&self) -> Position {
        self.src
    }

    pub fn dest(&self) -> Position {
        self.dest
    }

    pub fn captured(&self) -> u8 {
        self.captured
    }
}
