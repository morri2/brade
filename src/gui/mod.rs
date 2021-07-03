use crate::game::board::*;
use crate::game::color::*;
use crate::game::position::*;

/* TRASH TIER DEBUG GUI
q  w  e  r  t  y  |  u  i  o  p  å  ^
   #     #     #  |     #     #     15
                  |                 W
                  |
R                 |
15 #     #     #  |     #     #     #
a  s  d  f  g  h  |  j  k  l  ö  ä  *

*/
pub fn db_print(board: Board) {
    for i in 0..24{
        print!("{:<3}",i);
    }
    println!();
    for i in 0..24{
        if Board::is_closable(Position::Point(i)) {
            print!("{:<2}","+");
        } else {
            print!("{:<2}","-");
        }
        if i % 6 == 5 {
            print!("|")
        } else {
            print!(" ")
        }
    }
    println!();
    for i in 0..24{
        print!("{:<3}",board.get_marker_count(Position::Point(i)));
    }
    println!();
    for i in 0..24{
        print!("{:<3}",match board.get_color(Position::Point(i)) {
            Some(Color::White) => "W",
            Some(Color::Black) => "B",
            _ => ""
        });
    }
    println!();
    println!("                           bar: W {}|{} B", board.get_bar(Color::White), board.get_bar(Color::Black));
    println!();
}
