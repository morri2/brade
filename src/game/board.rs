use super::color::*;
use super::r#move::*;
use super::position::*;
const POINT_STEP: usize = 5; // bits to step to get next points in marker counter
const BOARD_MASK: u128 = (1 << 120)-1;

const POINT_DATA_MASK: u128 = 0b11111;
const MARKER_COUNT_MASK: u8 = 0b01111; 
const MARKER_COLOR_MASK: u8 = 0b10000;


const CLOSABLE: u128 = 0b0;

pub struct Board { //128 DRAFT AND ONLY 128
    board_data: u128,
    to_play: Color,
    bar: [u8;2],
    /*
    each point is represented as 5 bits where
    bit 0:3 is the number of markers, and bit 4 is the color (0 is White, 1 is Black)
    */
}

impl Board {
    fn new(board_data: u128, to_play: Color, bar: [u8;2]) -> Self{
        Board {
            to_play,
            board_data,
            bar,
        }
    }

    fn is_closable(pos: Position) -> bool{
        match pos {
            Position::Point(0..=10) => false,
            _ => true, // bar and out are closable just in case
        } 
    }

    fn flip(&mut self){
       self.board_data = ((self.board_data >> 12 * POINT_STEP) | (self.board_data << 12 * POINT_STEP)) & BOARD_MASK
    }

    fn get_point_data(&self, pos: Position) -> u8{
        match pos {
            Position::Point(point) => {(self.board_data >> (point * POINT_STEP)) as u8},
            _ => 0 // maybe option?
        }
    }

    fn get_marker_count(&self, pos: Position) -> u8{
        match pos {
            Position::Point(_) => {(self.get_point_data(pos) * MARKER_COUNT_MASK) as u8},
            _ => 0
        }
    }

    fn is_singleton(&self, pos: Position) -> bool {
        self.get_marker_count(pos) == 0
    }

    fn is_occupied(&self, pos: Position) -> bool {
        self.get_marker_count(pos) != 0
    }

    fn get_color(&self, pos: Position) -> Option<Color>{
        if self.is_occupied(pos) {
            if self.get_point_data(pos) & MARKER_COLOR_MASK == 0 /*White is index 0*/{ 
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
        let color_data = self.get_point_data(pos) & MARKER_COLOR_MASK;
        let count_data = count & MARKER_COUNT_MASK;
        self.set_point_data(pos, color_data | count_data);
    }

    fn set_marker_color(&mut self, pos:Position, color: Color) {
        let color_data = (color.index() as u8) << 4;
        let count_data = self.get_point_data(pos) & MARKER_COLOR_MASK;
        self.set_point_data(pos, color_data | count_data);
    }

    fn set_marker_color_count(&mut self, pos: Position, color: Color, count: u8) {
        let color_data = (color.index() as u8) << 4;
        let count_data = count & MARKER_COLOR_MASK;
        self.set_point_data(pos, color_data | count_data);
    }

    fn is_legal_dest(&self, pos: Position, color: Color) -> bool {
        if let Some(pcolor) = self.get_color(pos) {
            if pcolor == color {
                return /*closeable*/ false  
            } else {
                return self.is_singleton(pos) || /*sprängable*/ false
            }
        } else {
            true
        }
    }

    fn apply_submove(submove: Submove) {
        
    }
}
