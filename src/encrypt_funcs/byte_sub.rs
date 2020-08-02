use crate::utils::tables as tbl;

pub fn sub_bytes(state: Vec<u8>) -> Vec<u8> {
    let mut t_state: Vec<u8> = vec![0; state.len()];
    for s in state {
        t_state.push(tbl::s_box(s));
    }

    t_state
}

pub fn byte_sub(word: u32) -> u32 {
    let mut temp_vec: Vec<u8> = Vec::new();
    let mut temp_val: u32 = 0;
    temp_vec.push(((word & 0xFF000000) >> 24) as u8);
    temp_vec.push(((word & 0xFF0000) >> 16) as u8);
    temp_vec.push(((word & 0xFF00) >> 8) as u8);
    temp_vec.push((word & 0xFF) as u8);
    for t in 0..temp_vec.len() {
        temp_val |= tbl::s_box(temp_vec[t]) as u32;
        if t == 3 {
            break;
        }
        temp_val <<= 8;
    }

    temp_val
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::printer::print_state;

    #[test]
    pub fn test_sub_bytes() {
        let state: Vec<u8> = vec![0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30, 0xD4, 0xBF, 0x5D, 0x30];
        let state = sub_bytes(state);
        print_state(&state);
    }

}