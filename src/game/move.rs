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

impl Submove {}
