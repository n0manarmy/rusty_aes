use crate::prelude::*;

pub fn run(e: Decrypt, input: Vec<u8>) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let mut count = 0;
    let buf_size = e.block_size;

    while count < input.len() {
        let mut cipher_text = input[count..(count + buf_size)].to_vec();
        cipher_text = decrypt(&e.expanded_key, e.rounds, cipher_text); 
        buf.append(&mut cipher_text);
        count += buf_size;
    }

    padder::clear_padding(buf)
}

fn decrypt(expanded_key: &Vec<u8>, rounds: u32, input: Vec<u8>) -> Vec<u8> {
    let mut x = 0;
    // print!("{} -- iinput",x);
    // print_state(&input);
    // assert_eq!(&input, &test_tables::inv_cipher_128((x,"iinput")));

    // print!("transform input state");
    // let mut state = helper::transform_state(input);
    // print_state(&state);

    // print!("{} - ik_sch", x);
    // let ik_sch: Vec<u8> = helper::transform_state(
    //     helper::get_this_round_exp_key(self.rounds as usize, &self.expanded_key));
    let ik_sch: Vec<u8> = key_sch::get(rounds as usize, &expanded_key);
    // print_state(&ik_sch);
    // assert_eq!(&ik_sch, &test_tables::inv_cipher_128((x,"ik_sch")));
    // let ik_sch = helper::transform_state(ik_sch);

    // print!("start add round key");
    let mut state = add_round_key::xor(input, ik_sch);
    // print_state(&state);

    while x < (rounds - 1) {
        x += 1;
        // print!("\n{} - istart", x);
        // print_state(&state);
        // assert_eq!(&state, &test_tables::inv_cipher_128((x,"istart")));

        // print!("\n{} - is_row", x);
        state = inv_shift_rows::shift(state);
        // print_state(&state);
        // assert_eq!(&state, &test_tables::inv_cipher_128((x,"is_row")));

        // print!("\n{} - is_box", x);
        state = state.iter().map(|x| tables::inv_s_box(*x)).collect();
        // print_state(&state);
        // assert_eq!(&state, &test_tables::inv_cipher_128((x,"is_box")));

        // print!("\n{} - ik_sch", x);
        let ik_sch: Vec<u8> = key_sch::get((rounds - x) as usize, &expanded_key);
        // print_state(&ik_sch);
        // assert_eq!(&ik_sch, &test_tables::inv_cipher_128((x,"ik_sch")));

        // print!("\n{} - ik_add", x);
        state = add_round_key::xor(state, ik_sch);
        // print_state(&state);
        // assert_eq!(&state, &test_tables::inv_cipher_128((x,"ik_add")));

        // print!("\n{} - im_col", x);
        state = inv_mix_cols::mix(state);
        // print_state(&state);            
    }
    
    // x += 1;
    // print!("\n{} - inv is_row", x);
    state = inv_shift_rows::shift(state);
    // print_state(&state);

    // print!("\n{} - is_box", x);
    state = state.iter().map(|x| tables::inv_s_box(*x)).collect();
    // print_state(&state);

    // print!("ik_sch");
    // let ik_sch: Vec<u8> = helper::transform_state(
    let ik_sch: Vec<u8> = key_sch::get(0, &expanded_key);
    // print_state(&ik_sch);
    
    // print!("\n{} - ik_add", 0);
    state = add_round_key::xor(state, ik_sch);        
    // print_state(&state);

    state
}


#[cfg(test)]
mod tests {

use crate::decrypt::Decrypt;
use crate::encrypt::Encrypt;
use crate::aes_mode::*;

    #[test]
    pub fn test_decrypt_small() {
        let key: Vec<u8> = "THISTHEMYENCRKEY".as_bytes().to_vec();
        let text = String::from("testtesttesttesttest");
        let mut input: Vec<u8> = text.as_bytes().to_vec();
        input.push(0x80);
        input.push(0x80);
        let text: String = input.iter().map(|x| *x as char).collect();
        
        let mut e: Encrypt = Encrypt::ecb(key.clone(), AesMode::ECB);
        let cipher_text: Vec<u8> = e.encrypt(&input);

        let d: Decrypt = Decrypt::ecb(key);
        let results = d.decrypt(cipher_text);
        let results: String = results.iter().map(|x| *x as char).collect();

        assert_eq!(text, results);
    }
}