use super::*;
use crate::utils::tables as tbl;
use crate::utils::helper::xy_idx;

pub fn mix(state: Vec<u8>) -> Vec<u8> {
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


fn p_operate(state: &Vec<u8>, y: i32, row: usize) -> u8 {
    let t1 = galois_mult::gmul(state[xy_idx(0, y)], tbl::inv_m_mtrx(row, 0));
    let t2 = galois_mult::gmul(state[xy_idx(1, y)], tbl::inv_m_mtrx(row, 1));
    let t3 = galois_mult::gmul(state[xy_idx(2, y)], tbl::inv_m_mtrx(row, 2));
    let t4 = galois_mult::gmul(state[xy_idx(3, y)], tbl::inv_m_mtrx(row, 3));
    t1 ^ t2 ^ t3 ^ t4
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::printer::print_state;


    #[test]
    pub fn test_inv_mix_column() {
        // let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30];
        let state: Vec<u8> = vec![0x04, 0x66, 0x81, 0xe5, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,];
        // let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30];

        let state = mix(state);
        // print_state(&state);

        assert_eq!(state[xy_idx(0, 0)], 0xd4);
        assert_eq!(state[xy_idx(1, 0)], 0xbf);
        assert_eq!(state[xy_idx(2, 0)], 0x5d);
        assert_eq!(state[xy_idx(3, 0)], 0x30);
    }

    #[test]
    pub fn test_round_5_col_mix() {
        let state: Vec<u8> = vec![0x98, 0x16, 0xee, 0x74, 0x00, 0xf8, 0x7f, 0x55, 0x6b, 0x2c, 0x04, 0x9c, 0x8e, 0x5a, 0xd0, 0x36];
        let result: Vec<u8> = vec![0xe8, 0xda, 0xb6, 0x90, 0x14, 0x77, 0xd4, 0x65, 0x3f, 0xf7, 0xf5, 0xe2, 0xe7, 0x47, 0xdd, 0x4f];
        let state = mix(state);
        // print_state(&state);
        assert_eq!(state, result);
    }
}