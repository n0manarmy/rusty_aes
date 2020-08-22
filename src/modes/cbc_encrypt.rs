// use crate::utils::helper;
use crate::utils::{tables, padder};
use crate::encrypt::Encrypt;
// use crate::test_vals::test_tables::cipher_128;
use crate::utils::printer::print_state;
use crate::encrypt_funcs::{add_round_key, key_sch, mix_columns, shift_rows};

pub fn run(e: &Encrypt, input: &Vec<u8>, init_iv: Vec<u8>) -> Vec<u8> {
    let mut count = 0;
    let buf_size = e.block_size;
    let mut buf: Vec<u8> = Vec::new();
    let mut next_iv: Vec<u8> = Vec::new();
    let mut init_iv_applied = false;

    //loop through input until len reached
    while count < input.len() {
        let mut cipher_text: Vec<u8>;
        let end_next_chunk = count + buf_size;

        //look for padding here, if we're going to exceed then we pad
        if end_next_chunk >= input.len() {
            cipher_text = input[count..end_next_chunk].to_vec();
            cipher_text = padder::pad(cipher_text, buf_size);
            
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
        count += buf_size;
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
use crate::utils::{iv_builder, hex_encoders, printer::print_state};

#[test]
pub fn test_encrypt_128_cbc() {
    let input: Vec<u8> = vec![0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34];
    let cipher_key: Vec<u8> = vec![0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
    let result: Vec<u8> = vec![0x39, 0x02, 0xdc, 0x19, 0x25, 0xdc, 0x11, 0x6a, 0x84, 0x09, 0x85, 0x0b, 0x1d, 0xfb, 0x97, 0x32];
    let iv = iv_builder::get_iv(cipher_key.len());

    let encryptor = Encrypt::cbc(cipher_key, InitializationValue::None);
    let output: Vec<u8> = run(&encryptor, &input, iv);

    print_state(&output);

    assert_eq!(output, result);
}

#[test]
pub fn test_encrypt_plain_128_cbc() {
    let input = "00112233445566778899aabbccddeeff";
    let input: Vec<u8> = hex_encoders::str_to_hex_u8_buf(input);
    assert_eq!(input.len(), 16);
    let cipher = "000102030405060708090a0b0c0d0e0f";
    let cipher: Vec<u8> = hex_encoders::str_to_hex_u8_buf(cipher);

    let iv = iv_builder::get_iv(cipher.len());

    let result = "69c4e0d86a7b0430d8cdb78070b4c55a";

    let encryptor = Encrypt::cbc(cipher, InitializationValue::None);
    // let output: Vec<u8> = helper::transform_state(encryptor.encrypt(input));
    let output: Vec<u8> = run(&encryptor, &input, iv);
    print_state(&output);
    let output: String = output.iter().map(|x| format!("{:02x}", x)).collect();
    println!("output: {}", &output);
    assert_eq!(&output, result);
}

}