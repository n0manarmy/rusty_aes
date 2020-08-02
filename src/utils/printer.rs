use super::*;

pub fn print_state(state: &Vec<u8>) {
    const VERT: u8 = 1;
    let mut horz = 0;
    // horz = VERT;
    if horz == 0 {
        for x in 0..state.len() {
            if x % 4== 0 {
                // println!();
                // print!("\t{:02} - ", (x/4 + 1));
                print!(" ");
            }
            print!("{:02x}", state[x]);
        }
    } else {
        println!();
        let mut x = 0;
        let mut y = 0;
        print!("\t{} - |", y);
        for _z in 0..state.len() {
            if x == (state.len() / 4) as i32 {
                println!();
                x = 0;
                y += 1;
                print!("\t{} - |", y);
            }
            if y == (state.len() / 4) as i32 {
                y = 0;
            }
            print!("{:02x}|", state[helper::t_xy_idx(x, y)]);
            x += 1;
        }
    }
    println!();
}