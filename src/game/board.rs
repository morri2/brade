use super::color::*;
use super::position::*;
use super::r#move::*;
const POINT_STEP: usize = 5; // bits to step to get next points in marker counter
const BOARD_MASK: u128 = (1 << 120) - 1;

const POINT_DATA_MASK: u128 = 0b11111;
const MARKER_COUNT_MASK: u8 = 0b01111;
const MARKER_COLOR_MASK: u8 = 0b10000;

const HANDSOME_CHECK_MASK: u128 = 0xf << (23 * POINT_STEP)
    | 0xf << (22 * POINT_STEP)
    | 0xf << (21 * POINT_STEP)
    | 0xf << (20 * POINT_STEP)
    | 0xf << (19 * POINT_STEP)
    | 0xf << (23 * POINT_STEP);
const HANDSOME_COND_3_3_3_3_3: u128 = 3 << (23 * POINT_STEP)
    | 3 << (22 * POINT_STEP)
    | 3 << (21 * POINT_STEP)
    | 3 << (20 * POINT_STEP)
    | 3 << (19 * POINT_STEP)
    | 0x3 << (23 * POINT_STEP);
const HANDSOME_COND_5_5_5: u128 =
    5 << (23 * POINT_STEP) | 5 << (22 * POINT_STEP) | 5 << (21 * POINT_STEP);
const HANDSOME_COND_3_5_7: u128 =
    7 << (23 * POINT_STEP) | 5 << (22 * POINT_STEP) | 3 << (21 * POINT_STEP);
const HANDSOME_COND_15: u128 = 15 << (23 * POINT_STEP);

const CLOSABLE: u128 = 0b0;
#[derive(Clone)]
pub struct Board {
    board_data: u128,
    /*
    each point is represented as 5 bits where
    bit 0:3 is the number of markers, and bit 4 is the color (0 is White, 1 is Black)
    */
    to_play: Color,
    bar: [u8; 2],
    out: [u8; 2],
}

impl Board {
    fn new(board_data: u128, to_play: Color, bar: [u8; 2], out: [u8; 2]) -> Self {
        Board {
            to_play,
            board_data,
            bar,
            out,
        }
    }

    fn is_closable(pos: Position) -> bool {
        match pos {
            Position::Point(0..=10) => false,
            _ => true, // bar and out are closable just in case
        }
    }

    fn steps_to_out(pos: Position) -> usize {
        match pos {
            Position::Point(point) => 24 - point,
            _ => 0,
        }
    }

    fn flip(&mut self) {
        self.board_data = ((self.board_data >> 12 * POINT_STEP)
            | (self.board_data << 12 * POINT_STEP))
            & BOARD_MASK
    }

    fn get_point_data(&self, pos: Position) -> u8 {
        match pos {
            Position::Point(point) => (self.board_data >> (point * POINT_STEP)) as u8,
            _ => 0, // maybe option?
        }
    }

    fn get_marker_count(&self, pos: Position) -> u8 {
        match pos {
            Position::Point(_) => (self.get_point_data(pos) * MARKER_COUNT_MASK) as u8,
            _ => 0,
        }
    }

    fn is_singleton(&self, pos: Position) -> bool {
        self.get_marker_count(pos) == 0
    }

    fn is_occupied(&self, pos: Position) -> bool {
        self.get_marker_count(pos) != 0
    }

    fn get_color(&self, pos: Position) -> Option<Color> {
        if self.is_occupied(pos) {
            if self.get_point_data(pos) & MARKER_COLOR_MASK == 0
            /*White is index 0*/
            {
                Some(Color::White)
            } else {
                Some(Color::Black)
            }
        } else {
            None
        }
    }

    fn is_color(&self, pos: Position, color: Color) -> bool {
        self.get_color(pos) == Some(color)
    }

    fn set_point_data(&mut self, pos: Position, data: u8) {
        if let Position::Point(point) = pos {
            let data = self.get_point_data(pos);
            self.board_data &= !POINT_DATA_MASK << (POINT_STEP * point);
            self.board_data |= (data as u128) << (POINT_STEP * point);
        }
    }

    fn set_marker_count(&mut self, pos: Position, count: u8) {
        if count == 0 {
            return self.clear_point(pos); // return is () as of now, but might be changed to return count if needed.
        }
        let color_data = self.get_point_data(pos) & MARKER_COLOR_MASK;

        let count_data = count & MARKER_COUNT_MASK;
        self.set_point_data(pos, color_data | count_data);
    }

    fn set_marker_color(&mut self, pos: Position, color: Color) {
        let color_data = (color.index() as u8) << 4;
        let count_data = self.get_point_data(pos) & MARKER_COLOR_MASK;
        self.set_point_data(pos, color_data | count_data);
    }

    fn set_marker_color_count(&mut self, pos: Position, color: Color, count: u8) {
        if count == 0 {
            return self.clear_point(pos);
        }
        let color_data = (color.index() as u8) << 4;
        let count_data = count & MARKER_COLOR_MASK;
        self.set_point_data(pos, color_data | count_data);
    }

    fn clear_point(&mut self, pos: Position) {
        self.set_point_data(pos, 0)
    }

    fn increment_marker_count(&mut self, pos: Position, d: u8) {
        // sould be split into delta and +1/-1
        let count = self.get_marker_count(pos);
        self.set_marker_count(pos, count + d)
    }

    fn decrement_marker_count(&mut self, pos: Position, d: u8) {
        // sould be split into delta and +1/-1
        let count = self.get_marker_count(pos);

        if d >= count {
            return self.clear_point(pos);
        }

        self.set_marker_count(pos, count - d)
    }

    fn get_bar(&self, color: Color) -> u8 {
        self.bar[color.index()]
    }

    fn set_bar(&mut self, color: Color, count: u8) {
        self.bar[color.index()] = count
    }

    fn increment_bar(&mut self, color: Color, d: u8) {
        // sould be split into delta and +1/-1
        self.set_bar(color, self.get_bar(color) + d)
        // Check for jan!
    }

    fn decrment_bar(&mut self, color: Color, d: u8) {
        // sould be split into delta and +1/-1
        self.set_bar(color, self.get_bar(color) - d) // safety?
    }

    fn capture(&mut self, pos: Position) {
        if let Some(color) = self.get_color(pos) {
            let count = self.get_marker_count(pos); // rw if  clear_point gets return
            self.increment_bar(color, count);
            self.clear_point(pos);
        }
    }

    fn bar_is_empty(&self, color: Color) -> bool {
        self.get_bar(color) == 0
    }

    fn is_legal_dest(&self, pos: Position, color: Color) -> bool {
        if pos == Position::Out {
            self.can_move_out(color)
        } else if let Some(pcolor) = self.get_color(pos) {
            if pcolor == color {
                return Board::is_closable(pos);
            } else {
                return self.is_singleton(pos) || false; /*SPRÄNGABLE!*/
            }
        } else {
            true
        }
    }

    fn is_legal_dest_from_bar(&self, pos: Position, color: Color) -> bool {
        if let Some(pcolor) = self.get_color(pos) {
            if pcolor == color {
                return Board::is_closable(pos); // not acctually nessesery given standard ruleset
            } else {
                return self.is_singleton(pos) || false; /*SPRÄNGABLE! (from bar)*/
                // obs diffrent from normal sprängable
            }
        } else {
            true
        }
    }

    fn get_out(&self, color: Color) -> u8 {
        self.out[color.index()]
    }

    fn set_out(&mut self, color: Color, count: u8) {
        self.out[color.index()] = count;
    }

    fn increment_out(&mut self, color: Color, d: u8) {
        // sould be split into delta and +1/-1
        self.set_out(color, self.get_out(color) + d)
    }

    fn decrement_out(&mut self, color: Color, d: u8) {
        // sould be split into delta and +1/-1
        self.set_out(color, self.get_out(color) - d) // safety?
    }

    fn can_move_out(&self, color: Color) -> bool {
        // assumes to_play perspective, so maybe remove color arg?
        let mut count = self.get_out(color);
        for src_point in 18..24 {
            let point = Position::Point(src_point);
            if self.is_color(point, color) {
                count += self.get_marker_count(point);
            }
        }
        count == 15 // all markers are in last quadrant or out
    }

    fn all_out(&self, color: Color) -> bool {
        self.get_out(color) == 15
    }

    fn is_handsome(&self, color: Color) -> bool {
        self.board_data & HANDSOME_CHECK_MASK == HANDSOME_COND_3_3_3_3_3
            || self.board_data & HANDSOME_CHECK_MASK == HANDSOME_COND_5_5_5
            || self.board_data & HANDSOME_CHECK_MASK == HANDSOME_COND_3_5_7
            || self.board_data & HANDSOME_CHECK_MASK == HANDSOME_COND_15
    }

    fn generate_legal_submoves(&self, dice: [usize; 2]) {
        // make a tree?? :))
        let mut submoves: Vec<Submove> = Vec::new();
        let mut results: Vec<Board> = Vec::new();
        let color = self.to_play;
        if self.bar_is_empty(color) {
            for src_point in 0..24 {
                let src = Position::new(src_point); // could be Position::Point(src_point)
                if self.is_color(src, color) {
                    for die in dice.iter() {
                        let dest_point = src_point + die;
                        let dest = Position::new(dest_point);
                        if self.is_legal_dest(dest, color) {
                            let captures = self.get_marker_count(dest);
                            let submove = Submove::new(src, dest, captures);
                            submoves.push(submove);
                            results.push(self.clone()); // do the thing (apply)
                        }
                    }
                }
            }
        } else {
            for die in dice.iter() {
                let dest = Position::new(*die);
                if self.is_legal_dest_from_bar(dest, color) {
                    let captures = self.get_marker_count(dest);
                    let submove = Submove::new(Position::Bar, dest, captures);
                    submoves.push(submove);
                    results.push(self.clone()); // do the thing (apply)
                }
            }
        }
    }

    fn apply_submove(&mut self, submove: Submove) {
        match submove.src() {
            // decriment src pos
            Position::Bar => self.decrment_bar(self.to_play, 1),
            Position::Point(_) => self.decrement_marker_count(submove.src(), 1),
            Position::Out => panic!("Atmepted submove from Out!"),
        }

        assert_eq!(submove.captured(), self.get_marker_count(submove.dest())); // sussy behaviour
        self.capture(submove.dest()); // capture

        match submove.dest() {
            Position::Out => self.increment_out(self.to_play, 1),
            Position::Point(_) => self.increment_marker_count(submove.dest(), 1),
            Position::Bar => panic!("Atmepted submove to Bar!"),
        }
    }

    fn clone_apply_submove(&self, submove: Submove) -> Self {
        let mut res = self.clone();
        res.apply_submove(submove);
        res
    }

    fn undo_submove(&mut self, submove: Submove) {
        //TODO
    }

    fn clone_undo_submove(&self, submove: Submove) -> Self {
        let mut res = self.clone();
        res.undo_submove(submove);
        res
    }
}
