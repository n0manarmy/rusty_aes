use super::*;

pub fn print_hex_aligned(val: &Vec<u8>) {
    println!();
    for z in 0..val.len() {
        if z % 16 == 0 {
            println!();
        }
        // if y == (val.len() / 4) as i32 {
        //     y = 0;
        // }
        print!("{:02x}", val[z]);
    }
    println!();
}

pub fn print_state(state: &Vec<u8>) {
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
    println!();
}