/* TRASH TIER DEBUG GUI

q  w  e  r  t  y  |  u  i  o  p  å  ^
                  |                 15
                  |                 W
                  |
R                 |
15 #     #     #  |     #     #     #
a  s  d  f  g  h  |  j  k  l  ö  ä  *

*/

fn cnt_str(n: u8) -> String {
    if n == 0 {
        format!("{:<3}", "#")
    } else {
        format!("{:<3}", n)
    }
}

pub fn db_disp(b: [u8; 24]) {
    print!(
        "    
    q  w  e  r  t  y  |  u  i  o  p  å  ^ \n
    {}{}{}{}{}{}|  {}{}{}{}{}{}\n
                      |                 W \n
                      |                   \n
    R                 |                   \n
    {}{}{}{}{}{}|  {}{}{}{}{}{}\n
    a  s  d  f  g  h  |  j  k  l  ö  ä  *",
        cnt_str(b[11]),
        cnt_str(b[10]),
        cnt_str(b[9]),
        cnt_str(b[8]),
        cnt_str(b[7]),
        cnt_str(b[6]),
        cnt_str(b[5]),
        cnt_str(b[4]),
        cnt_str(b[3]),
        cnt_str(b[2]),
        cnt_str(b[1]),
        cnt_str(b[0]),
        cnt_str(b[12]),
        cnt_str(b[13]),
        cnt_str(b[14]),
        cnt_str(b[15]),
        cnt_str(b[16]),
        cnt_str(b[17]),
        cnt_str(b[18]),
        cnt_str(b[19]),
        cnt_str(b[20]),
        cnt_str(b[21]),
        cnt_str(b[22]),
        cnt_str(b[23])
    )
}
