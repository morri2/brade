use super::color::*;
use super::r#move::*;
const POINT_STEP: usize = 5; // bits to step to get next points in marker counter
const BOARD_MASK: u128 = (1 << 120)-1;

const MARKER_COUNT_MASK: u128 = 0b1111; 
const MARKER_COLOR_MASK: u128 = 0b10000;

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

    fn flip(&mut self){
       self.board_data = ((self.board_data >> 12 * POINT_STEP) | (self.board_data << 12 * POINT_STEP)) & BOARD_MASK
    }

    fn get_point_data(&self, point: usize) -> u128{
        assert!(point < 24);
        self.board_data >> (point * POINT_STEP)
    }

    fn get_marker_count(&self, point: usize) -> u8{
        (self.get_point_data(point) * MARKER_COUNT_MASK) as u8
    }

    fn is_singleton(&self, point: usize) -> bool {
        self.get_marker_count(point) == 1
    }

    fn is_occupied(&self, point: usize) -> bool {
        self.get_marker_count(point) != 0
    }

    fn get_color(&self, point: usize) -> Option<Color>{
        if self.is_occupied(point) {
            if self.get_point_data(point) & MARKER_COLOR_MASK == 0 /*White is index 0*/{ 
                Some(Color::White)
            } else {
                Some(Color::Black)
            }
        } else {
            None
        }
    }

    fn is_color(&self, point: usize, color: Color) -> bool {
        self.get_color(point) == Some(color)
    }

    fn is_legal_dest(&self, point: usize, color: Color) -> bool {
        if let Some(pcolor) = self.get_color(point) {
            if pcolor == color {
                return /*closeable*/ false  
            } else {
                return self.is_singleton(point) || /*sprängable*/ false
            }
        } else {
            true
        }
    }

    fn apply_submove(submove: Submove) {

        
    }
}
