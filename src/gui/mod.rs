/* TRASH TIER DEBUG GUI

q  w  e  r  t  y  |  u  i  o  p  å  ^
                  |                 15
                  |                 W
                  |
R                 |
15 #     #     #  |     #     #     #
a  s  d  f  g  h  |  j  k  l  ö  ä  *

*/

fn marker_count_fmt(n: u8) -> String {
    if n == 0 {
        format!("{:<3}", "#")
    } else {
        format!("{:<3}", n)
    }
}

pub fn db_disp(board: [u8; 24]) {
    print!("    BRÄDE!!!!! \n    
    q  w  e  r  t  y  |  u  i  o  p  å  ^ \n
    {}{}{}{}{}{}|  {}{}{}{}{}{}\n
                      |                 W \n
                      |                   \n
    R                 |                   \n
    {}{}{}{}{}{}|  {}{}{}{}{}{}\n
    a  s  d  f  g  h  |  j  k  l  ö  ä  *",
        marker_count_fmt(board[11]),
        marker_count_fmt(board[10]),
        marker_count_fmt(board[9]),
        marker_count_fmt(board[8]),
        marker_count_fmt(board[7]),
        marker_count_fmt(board[6]),
        marker_count_fmt(board[5]),
        marker_count_fmt(board[4]),
        marker_count_fmt(board[3]),
        marker_count_fmt(board[2]),
        marker_count_fmt(board[1]),
        marker_count_fmt(board[0]),
        marker_count_fmt(board[12]),
        marker_count_fmt(board[13]),
        marker_count_fmt(board[14]),
        marker_count_fmt(board[15]),
        marker_count_fmt(board[16]),
        marker_count_fmt(board[17]),
        marker_count_fmt(board[18]),
        marker_count_fmt(board[19]),
        marker_count_fmt(board[20]),
        marker_count_fmt(board[21]),
        marker_count_fmt(board[22]),
        marker_count_fmt(board[23])
    )
}
