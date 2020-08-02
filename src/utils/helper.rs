use super::*;

pub fn xy_idx(x: i32, y: i32) -> usize {
    // println!("x: {} y: {}", x, y);
    (y as usize * 4) + x as usize
}

pub fn t_xy_idx(x: i32, y: i32) -> usize {
    match xy_idx(x, y) {
        0  => 0,
        1  => 4,
        2  => 8,
        3  => 12,
        4  => 1,
        5  => 5,
        6  => 9,
        7  => 13,
        8  => 2,
        9  => 6,
        10 => 10,
        11 => 14,
        12 => 3,
        13 => 7,
        14 => 11,
        15 => 15,
        _  => panic!("error in lookup in t_xy_idx table"),
    }
}



pub fn overflow_check(x: u8, y: u8) -> u8 {
    // println!("overflow_check ---- {:02} {:02} {:02x} {:02x}", x, y, x, y);
    match x.checked_add(y) {
        Some(val) => return val,
        None => return ((x as u32 + y as u32) % 0xff) as u8,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_xy_idx() {
        assert_eq!(xy_idx(0, 1), 4)
    }

    #[test]
    pub fn test_t_xy_idx() {
        let test = vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
        let mut t_state: Vec<u8> = vec![0;test.len()];
        let mut x = 0;
        let mut y = 0;
        for z in 0..t_state.len() {
            if x == (test.len() / 4) as i32 {
                x = 0;
                y += 1;
            }
            if y == (test.len() / 4) as i32 {
                y = 0;
            }
            t_state[z] = test[t_xy_idx(x, y)];
            x += 1;
        }

        printer::print_state(&t_state);
    }


}