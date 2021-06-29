/* TRASH TIER DEBUG GUI

q  w  e  r  t  y  |  u  i  o  p  å  ^
                  |                 15
                  |                 W
                  |
R                 |
15 #     #     #  |     #     #     #
a  s  d  f  g  h  |  j  k  l  ö  ä  *

*/

fn cnt_str(n: u8) -> String{
    if n == 0 {
        format!("{:<3}", "#")
    } else {
        format!("{:<3}", n)
    }
}


pub fn db_disp(b: [u8;24]) {
   print!("q  w  e  r  t  y  |  u  i  o  p  å  ^ \n
    {}{}{}{}{}{}|{}{}{}{}{}{}\n
                      |                 W \n
                      |                   \n
    R                 |                   \n
    {}{}{}{}{}{}|{}{}{}{}{}{}\n
    a  s  d  f  g  h  |  j  k  l  ö  ä  *", b[0],b[1],b[2],b[3],b[4],b[5],
    b[6],b[7],b[8],b[9],b[10],b[11],
    b[12],b[13],b[14],b[15],b[16],b[17],
    b[18],b[19],b[20],b[21],b[22],b[23])
}
