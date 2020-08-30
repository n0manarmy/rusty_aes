// use crate::utils::helper;
use crate::utils::{tables, padder};
use crate::encrypt::Encrypt;
// use crate::test_vals::test_tables::cipher_128;
use crate::utils::printer::print_state;
use crate::encrypt_funcs::{add_round_key, key_sch, mix_columns, shift_rows};

pub fn run(e: &Encrypt, input: &Vec<u8>, init_iv: Vec<u8>) -> Vec<u8> {
    let mut count = 0;
    let mut buf: Vec<u8> = Vec::new();
    let mut next_iv: Vec<u8> = Vec::new();
    let mut init_iv_applied = false;

    //loop through input until len reached
    while count < input.len() {
        let mut cipher_text: Vec<u8>;
        let end_next_chunk = count + 16;

        //look for padding here, if we're going to exceed then we pad
        if end_next_chunk >= input.len() {
            cipher_text = input[count..input.len()].to_vec();
            cipher_text = padder::pad(cipher_text);
            
            // xor IV with initial state
            if init_iv_applied == false {
                cipher_text = init_iv.iter().zip(cipher_text.iter()).map(|(a,b)| a ^ b).collect::<Vec<u8>>();
                init_iv_applied = true;
            } else {
                cipher_text = next_iv.iter().zip(cipher_text.iter()).map(|(a,b)| a ^ b).collect::<Vec<u8>>();
            }
            
            cipher_text = encrypt(&e.expanded_key, e.rounds, cipher_text);
            next_iv = cipher_text.clone();

            buf.append(&mut cipher_text);
        }
        else {
            cipher_text = input[count..end_next_chunk].to_vec();

            // xor IV with initial state
            if init_iv_applied == false {
                cipher_text = init_iv.iter().zip(cipher_text.iter()).map(|(a,b)| a ^ b).collect::<Vec<u8>>();
                init_iv_applied = true;
            } else {
                cipher_text = next_iv.iter().zip(cipher_text.iter()).map(|(a,b)| a ^ b).collect::<Vec<u8>>();
            }

            cipher_text = encrypt(&e.expanded_key, e.rounds, cipher_text);
            next_iv = cipher_text.clone();

            buf.append(&mut cipher_text);
        }
        count += 16;
    }

    buf
}

fn encrypt(expanded_key: &Vec<u8>, rounds: u32, input: Vec<u8>) -> Vec<u8> {
    let mut x = 0;
    // print!("{} - input", x);
    // print_state(&input);

    // let mut state = helper::transform_state(input);
    let mut state = input;

    // print!("{} - k_sch", x);
    let ik_sch: Vec<u8> = key_sch::get(0, expanded_key);
    // print_state(&ik_sch);
    state = add_round_key::xor(state, ik_sch);

    while x < (rounds - 1) {
        x += 1;
        // print!("\n{} - start", x);
        // print_state(&state);

        // print!("\n{} - s_box", x);
        state = state.iter().map(|x| tables::s_box(*x)).collect();
        // print_state(&state);

        // print!("\n{} - s_row", x);
        state = shift_rows::shift(state);
        // print_state(&state);

        // print!("\n{} - m_col", x);
        // state = mix_columns::table_mix(state);
        state = mix_columns::table_mix(state);
        // print_state(&state);

        let ik_sch: Vec<u8> = key_sch::get(x as usize, expanded_key);
        // print_state(&ik_sch);
                
        // print!("\n{} - k_add", x);
        state = add_round_key::xor(state, ik_sch);
        // print_state(&state);
    }

    x += 1;
    // print!("\n{} - s_box", x);
    state = state.iter().map(|x| tables::s_box(*x)).collect();
    // print_state(&state);

    // print!("\n{} - s_row", x);
    state = shift_rows::shift(state);
    // print_state(&state);
    
    // print!("\n{} - ik_sch", x);
    let ik_sch: Vec<u8> = key_sch::get(rounds as usize, expanded_key);
    // print_state(&ik_sch);

    state = add_round_key::xor(state, ik_sch);

    state

}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::encrypt::InitializationValue;
    use crate::utils::{iv_builder, hex_encoders};

    #[test]
    pub fn test_encrypt_128_cbc() {
        let key: Vec<u8> = vec![0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
        let iv: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f];

        let plain_text: Vec<u8> = vec![0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96, 0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93, 0x17, 0x2a,
                                        0xae, 0x2d, 0x8a, 0x57, 0x1e, 0x03, 0xac, 0x9c, 0x9e, 0xb7, 0x6f, 0xac, 0x45, 0xaf, 0x8e, 0x51,
                                        0x30, 0xc8, 0x1c, 0x46, 0xa3, 0x5c, 0xe4, 0x11, 0xe5, 0xfb, 0xc1, 0x19, 0x1a, 0x0a, 0x52, 0xef,
                                        0xf6, 0x9f, 0x24, 0x45, 0xdf, 0x4f, 0x9b, 0x17, 0xad, 0x2b, 0x41, 0x7b, 0xe6, 0x6c, 0x37, 0x10];
        let cipher_text: Vec<u8> = vec![0x76, 0x49, 0xab, 0xac, 0x81, 0x19, 0xb2, 0x46, 0xce, 0xe9, 0x8e, 0x9b, 0x12, 0xe9, 0x19, 0x7d,
                                        0x50, 0x86, 0xcb, 0x9b, 0x50, 0x72, 0x19, 0xee, 0x95, 0xdb, 0x11, 0x3a, 0x91, 0x76, 0x78, 0xb2,
                                        0x73, 0xbe, 0xd6, 0xb8, 0xe3, 0xc1, 0x74, 0x3b, 0x71, 0x16, 0xe6, 0x9e, 0x22, 0x22, 0x95, 0x16,
                                        0x3f, 0xf1, 0xca, 0xa1, 0x68, 0x1f, 0xac, 0x09, 0x12, 0x0e, 0xca, 0x30, 0x75, 0x86, 0xe1, 0xa7];

        let mut e = Encrypt::cbc(key, InitializationValue::IV(iv));
        assert_eq!(e.encrypt(&plain_text), cipher_text);
    }
}