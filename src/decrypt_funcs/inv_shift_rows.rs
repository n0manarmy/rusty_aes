pub fn shift(state: Vec<u8>) -> Vec<u8> {

    let mut t_state: Vec<u8> = vec![0; 16];
    // t_state[0] = state[0];
    // t_state[1] = state[1];
    // t_state[2] = state[2];
    // t_state[3] = state[3];

    // t_state[4] = state[7];
    // t_state[5] = state[4];
    // t_state[6] = state[5];
    // t_state[7] = state[6];
    
    // t_state[8] = state[10];
    // t_state[9] = state[11];
    // t_state[10] = state[8];
    // t_state[11] = state[9];
    
    // t_state[12] = state[13];
    // t_state[13] = state[14];
    // t_state[14] = state[15];
    // t_state[15] = state[12];

    // t_state[0] = state[0];
    // t_state[1] = state[15];
    // t_state[2] = state[10];
    // t_state[3] = state[7];

    // t_state[4] = state[4];
    // t_state[5] = state[1];
    // t_state[6] = state[14];
    // t_state[7] = state[11];
    
    // t_state[8] = state[8];
    // t_state[9] = state[5];
    // t_state[10] = state[2];
    // t_state[11] = state[15];
    
    // t_state[12] = state[12];
    // t_state[13] = state[9];
    // t_state[14] = state[6];
    // t_state[15] = state[3];

    t_state[0] = state[0];
    t_state[1] = state[13];
    t_state[2] = state[10];
    t_state[3] = state[7];

    t_state[4] = state[4];
    t_state[5] = state[1];
    t_state[6] = state[14];
    t_state[7] = state[11];
    
    t_state[8] = state[8];
    t_state[9] = state[5];
    t_state[10] = state[2];
    t_state[11] = state[15];
    
    t_state[12] = state[12];
    t_state[13] = state[9];
    t_state[14] = state[6];
    t_state[15] = state[3];

    t_state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_inv_s_row() {
        // let state: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
        let s2: Vec<u8> = vec![0x7a, 0xd5, 0xfd, 0xa7, 0x89, 0xef, 0x4e, 0x27, 0x2b, 0xca, 0x10, 0x0b, 0x3d, 0x9f, 0xf5, 0x9f];
        let s2_r: Vec<u8> = vec![0x7a, 0x9f, 0x10, 0x27, 0x89, 0xd5, 0xf5, 0x0b, 0x2b, 0xef, 0xfd, 0x9f, 0x3d, 0xca, 0x4e, 0xa7];
        // let state = shift(state);
        // print_state(&state);

        // print_state(&s2);
        // let s2 = inv_shift_rows(transform_state(s2));
        // print_state(&s2);
        // let s2 = transform_state(s2);
        // print_state(&s2);
        // print_state(&s2_r);
        assert_eq!(shift(s2), s2_r);
    }
}