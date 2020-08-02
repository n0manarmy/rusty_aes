use crate::utils::helper::t_xy_idx;

pub fn shift(state: Vec<u8>) -> Vec<u8> {

    let mut t_state: Vec<u8> = vec![0; state.len()];
    // let mut x = 0;
    // let mut y = 0;
    // for z in 0..t_state.len() {
    //     if x == (state.len() / 4) as i32 {
    //         x = 0;
    //     }
    //     if y == (state.len() / 4) as i32 {
    //         y = 0;
    //     }
    //     t_state[z] = state[t_xy_idx(x, y)];
    //     x += 1;
    //     y += 1;
    // }
    // t_state[0] = state[0];
    // t_state[1] = state[1];
    // t_state[2] = state[2];
    // t_state[3] = state[3];

    // t_state[4] = state[5];
    // t_state[5] = state[6];
    // t_state[6] = state[7];
    // t_state[7] = state[4];
    
    // t_state[8] = state[10];
    // t_state[9] = state[11];
    // t_state[10] = state[8];
    // t_state[11] = state[9];
    
    // t_state[12] = state[15];
    // t_state[13] = state[12];
    // t_state[14] = state[13];
    // t_state[15] = state[14];

    // t_state[0] = state[0];
    // t_state[1] = state[4];
    // t_state[2] = state[8];
    // t_state[3] = state[12];

    // t_state[4] = state[5];
    // t_state[5] = state[9];
    // t_state[6] = state[13];
    // t_state[7] = state[1];
    
    // t_state[8] = state[10];
    // t_state[9] = state[14];
    // t_state[10] = state[2];
    // t_state[11] = state[6];
    
    // t_state[12] = state[15];
    // t_state[13] = state[3];
    // t_state[14] = state[7];
    // t_state[15] = state[11];

    t_state[0] = state[0];
    t_state[1] = state[5];
    t_state[2] = state[10];
    t_state[3] = state[15];

    t_state[4] = state[4];
    t_state[5] = state[9];
    t_state[6] = state[14];
    t_state[7] = state[3];
    
    t_state[8] = state[8];
    t_state[9] = state[13];
    t_state[10] = state[2];
    t_state[11] = state[7];
    
    t_state[12] = state[12];
    t_state[13] = state[1];
    t_state[14] = state[6];
    t_state[15] = state[11];
    
    t_state
}

#[cfg(test)]
mod tests {
    
    use super::*;
    use crate::utils::helper;
    use crate::utils::printer::print_state;
    #[test]
    pub fn test_shift_row() {
        let state: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
        print_state(&state);
        let state = shift(state);
        print_state(&state);
    }
}