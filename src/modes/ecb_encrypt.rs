use crate::utils::tables;
use crate::encrypt::Encrypt;
use crate::utils::{padder, printer::*};
use crate::encrypt_funcs::{add_round_key, key_sch, mix_columns, shift_rows};

/// Run the encryption program with the initialized encryption engine and input file
/// 
/// # Arguments
/// 
/// * `Encrypt` engine containing the initiailized values for the procedures
/// * `&Vec<u8>` containing the input to be encrypted
/// 
/// # Examples
/// 
/// ```
/// let e: Encrypt = Encrypt::new();
/// let input: Vec<u8> = "Encrypt me".as_bytes().to_vec();
/// e.run(&e, &input);
pub fn run(e: &Encrypt, input: &Vec<u8>) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let mut input_consumed = 0;
    let block_size = e.block_size;

    while input_consumed < input.len() {
        let mut cipher_text: Vec<u8>;
        let end_next_chunk = input_consumed + block_size;

        if end_next_chunk >= input.len() {
            //fill cipher_text with last of input
            cipher_text = input[input_consumed..input.len()].to_vec();
            //pad cipher_text with padding bits
            cipher_text = padder::pad(cipher_text, block_size);
            //check for single character padding that pads an additional block
            if cipher_text.len() == block_size * 2 {
                //we encrypt the padded cipher_text
                let (first, second) = cipher_text.split_at(block_size);
                let first = encrypt(&e.expanded_key, e.rounds, first.to_vec());
                let second = encrypt(&e.expanded_key, e.rounds, second.to_vec());
                cipher_text = [first, second].concat();
                
            } else {
                //we encrypt the padded cipher_text
                cipher_text = encrypt(&e.expanded_key, e.rounds, cipher_text);
            }
            
        }
        else {
            cipher_text = input[input_consumed..end_next_chunk].to_vec();
            cipher_text = encrypt(&e.expanded_key, e.rounds, cipher_text); 
        }

        buf.append(&mut cipher_text);
        input_consumed += block_size;
    }

    buf
}

fn encrypt(expanded_key: &Vec<u8>, rounds: u32, input: Vec<u8>) -> Vec<u8> {
    let mut x = 0;
    // print!("{} - input", x);
    // print_state(&input);
    // assert_eq!(&input, &cipher_128((x, "input")));

    // let mut state = helper::transform_state(input);
    let mut state = input;

    // print!("{} - k_sch", x);
    let ik_sch: Vec<u8> = key_sch::get(0, expanded_key);
    // print_state(&ik_sch);
    // assert_eq!(&ik_sch, &cipher_128((x, "k_sch")));
    state = add_round_key::xor(state, ik_sch);

    while x < (rounds - 1) {
        x += 1;
        // print!("\n{} - start", x);
        // print_state(&state);
        // assert_eq!(&state, &cipher_128((x, "start")));

        // print!("\n{} - s_box", x);
        state = state.iter().map(|x| tables::s_box(*x)).collect();
        // print_state(&state);
        // assert_eq!(&state, &cipher_128((x, "s_box")));

        // print!("\n{} - s_row", x);
        state = shift_rows::shift(state);
        // print_state(&state);
        // assert_eq!(&state, &cipher_128((x, "s_row")));

        // print!("\n{} - m_col", x);
        // state = mix_columns::table_mix(state);
        state = mix_columns::table_mix(state);
        // print_state(&state);
        // assert_eq!(&state, &cipher_128((x, "m_col")));

        // print!("\n{} - k_sch", x);
        // let ik_sch: Vec<u8> = helper::transform_state(
            // helper::get_this_round_exp_key(x as usize, &self.expanded_key));
        
        let ik_sch: Vec<u8> = key_sch::get(x as usize, expanded_key);
        // print_state(&ik_sch);
        // assert_eq!(&ik_sch, &cipher_128((x, "k_sch")));

        // print!("\n{} - k_add", x);
        state = add_round_key::xor(state, ik_sch);
        // print_state(&state);
    }

    // x += 1;
    // print!("\n{} - s_box", self.rounds);
    state = state.iter().map(|x| tables::s_box(*x)).collect();
    // print_state(&state);
    // assert_eq!(&state, &cipher_128((x, "s_box")));

    // print!("\n{} - s_row", self.rounds);
    state = shift_rows::shift(state);
    // print_state(&state);
    // assert_eq!(&state, &cipher_128((x, "s_row")));

    // print!("k_sch");
    // let ik_sch: Vec<u8> = helper::transform_state(helper::get_this_round_exp_key(self.rounds as usize, &self.expanded_key));
    let ik_sch: Vec<u8> = key_sch::get(rounds as usize, expanded_key);
    // print_state(&ik_sch);
    // assert_eq!(&ik_sch, &cipher_128((x, "k_sch")));

    state = add_round_key::xor(state, ik_sch);        

    state
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::{hex_encoders, printer};

    #[test]
    pub fn test_encrypt_128_ecb() {
        let input: Vec<u8> = vec![0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34];
        let key: Vec<u8> = vec![0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
        let result: Vec<u8> = vec![0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a, 0x0b, 0x32]; 

        let mut e = Encrypt::ecb(key);
        let output: Vec<u8> = e.encrypt(&input);

        assert_eq!(output, result);
    }

    #[test]
    pub fn test_encrypt_plain_128_ecb() {
        let input = "00112233445566778899aabbccddeeff";
        let input: Vec<u8> = hex_encoders::str_to_hex_u8_buf(input);
        assert_eq!(input.len(), 16);
        let cipher = "000102030405060708090a0b0c0d0e0f";
        let cipher: Vec<u8> = hex_encoders::str_to_hex_u8_buf(cipher);

        let result = "69c4e0d86a7b0430d8cdb78070b4c55a";

        let mut e = Encrypt::ecb(cipher);
        // let output: Vec<u8> = helper::transform_state(encryptor.encrypt(input));
        let output: Vec<u8> = e.encrypt(&input);
        printer::print_state(&output);
        let output: String = output.iter().map(|x| format!("{:02x}", x)).collect();
        println!("output: {}", &output);
        assert_eq!(&output, result);
    }

}