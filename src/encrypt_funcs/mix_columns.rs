use crate::prelude::*;

pub fn table_mix(state: Vec<u8>) -> Vec<u8> {
    // println!("##### start mix column");
    let mut t_state: Vec<u8> = vec![0;state.len()];
    let mut s_pos: i32 = 0;
    let mut y = 0;

    while s_pos < state.len() as i32 {

        t_state[xy_idx(0, y)] = p_operate(&state, y, 0);
        t_state[xy_idx(1, y)] = p_operate(&state, y, 1);
        t_state[xy_idx(2, y)] = p_operate(&state, y, 2);
        t_state[xy_idx(3, y)] = p_operate(&state, y, 3);
        s_pos += 4;
        y += 1;
    }

    t_state
}

fn p_operate(state: &Vec<u8>, y: i32, col: usize) -> u8 {
    let t1 = galois_mult::gmul(state[xy_idx(0, y)], m_mtrx(0, col));
    let t2 = galois_mult::gmul(state[xy_idx(1, y)], m_mtrx(1, col));
    let t3 = galois_mult::gmul(state[xy_idx(2, y)], m_mtrx(2, col));
    let t4 = galois_mult::gmul(state[xy_idx(3, y)], m_mtrx(3, col));
    // println!("t1: {:02x} t2: {:02x} t3: {:02x} t4: {:02x} ", t1, t2, t3, t4);
    t1 ^ t2 ^ t3 ^ t4
}

// fn operate(state: &Vec<u8>, y: i32, col: usize) -> u8 {
//     let t1 = overflow_check(tbl::l_box(state[xy_idx(0, y)]), tbl::l_box(tbl::m_mtrx(0, col)));
//     let t2 = overflow_check(tbl::l_box(state[xy_idx(1, y)]), tbl::l_box(tbl::m_mtrx(1, col)));
//     let t3 = overflow_check(tbl::l_box(state[xy_idx(2, y)]), tbl::l_box(tbl::m_mtrx(2, col)));
//     let t4 = overflow_check(tbl::l_box(state[xy_idx(3, y)]), tbl::l_box(tbl::m_mtrx(3, col)));
//     tbl::e_box(t1) ^ tbl::e_box(t2) ^ tbl::e_box(t3) ^ tbl::e_box(t4)
// }


#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    pub fn test_mix_column() {
        let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30];
        // let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,];
        // let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30];
        // let state = transform_state(state);

        let state = table_mix(state);
        // print_state(&state);

        assert_eq!(state[xy_idx(0, 0)], 0x04);
        assert_eq!(state[xy_idx(1, 0)], 0x66);
        assert_eq!(state[xy_idx(2, 0)], 0x81);
        assert_eq!(state[xy_idx(3, 0)], 0xe5);
    }
}