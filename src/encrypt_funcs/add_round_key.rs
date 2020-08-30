// use crate::utils::helper::t_xy_idx;

pub fn xor(mut state: Vec<u8>, cipher: Vec<u8>) -> Vec<u8>{
    let iter = state.iter().zip(cipher.iter());
    state = iter.map(|(s, e)| s ^ e).collect::<Vec<u8>>();

    state

    // let mut x = 0;
    // let mut y = 0;
    // let mut t_state: Vec<u8> = vec![0; state.len()];
    // for z in 0..state.len() {
    //     if x == (state.len() / 4) as i32 {
    //         x = 0;
    //         y += 1;
    //     }
    //     t_state[z] = state[t_xy_idx(x, y)] ^ cipher[t_xy_idx(x, y)];
    //     x += 1;
    // }

    // t_state
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::printer::print_state;
    use crate::key_expander;
    use crate::encrypt_funcs::key_sch;
    
    #[test]
    pub fn test_single_add_round() {
        let state: Vec<u8> =  vec![0x54, 0x77, 0x6F, 0x20, 0x4F, 0x6E, 0x65, 0x20, 0x4E, 0x69, 0x6E, 0x65, 0x20, 0x54, 0x77, 0x6F];
        let cipher: Vec<u8> = vec![0x54, 0x68, 0x61, 0x74, 0x73, 0x20, 0x6D, 0x79, 0x20, 0x4B, 0x75, 0x6E, 0x67, 0x20, 0x46, 0x75];
        let iter = state.iter().zip(cipher.iter());

        let results: Vec<u8> = iter.map(|(s,c)| s ^ c).collect();
        // print_state(&results);
    }

}