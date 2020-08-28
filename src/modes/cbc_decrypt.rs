use crate::decrypt::Decrypt;
use crate::utils::{tables, padder};
use crate::decrypt_funcs::{inv_mix_cols, inv_shift_rows};
use crate::encrypt_funcs::{key_sch, add_round_key};

pub fn run(e: &Decrypt, input: Vec<u8>) -> Vec<u8> {
    let mut count = 0;
    let buf_size = e.block_size;
    let mut buf: Vec<u8> = Vec::new();
    let mut init_iv_applied = false;

    //loop through input until len reached
    while count < input.len() {

        if count + buf_size >= input.len() {
            let mut cipher_text = input[count..input.len()].to_vec();
            //apply padding
            cipher_text = padder::pad(cipher_text, buf_size);

            cipher_text = decrypt(&e.expanded_key, e.rounds, cipher_text);
            
            if init_iv_applied == false {
                cipher_text = e.iv.iter().zip(cipher_text.iter()).map(|(a,b)| a ^ b).collect::<Vec<u8>>();
                init_iv_applied = true;
            } else {
                let previous_cipher_text = input[(count - buf_size)..count].to_vec();
                cipher_text = previous_cipher_text.iter().zip(cipher_text.iter()).map(|(a,b)| a ^ b).collect::<Vec<u8>>();

                // cipher_text = next_iv.iter().zip(cipher_text.iter()).map(|(a,b)| a ^ b).collect::<Vec<u8>>();
            }
            
            buf.append(&mut cipher_text);
        } else {
            let mut cipher_text = input[count..(count + buf_size)].to_vec();            

            cipher_text = decrypt(&e.expanded_key, e.rounds, cipher_text);
            
            if init_iv_applied == false {
                cipher_text = e.iv.iter().zip(cipher_text.iter()).map(|(a,b)| a ^ b).collect::<Vec<u8>>();
                init_iv_applied = true;
            } else {
                let previous_cipher_text = input[(count - buf_size)..count].to_vec();
                cipher_text = previous_cipher_text.iter().zip(cipher_text.iter()).map(|(a,b)| a ^ b).collect::<Vec<u8>>();
            }

            buf.append(&mut cipher_text);
        }
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
    let ik_sch: Vec<u8> = key_sch::get(rounds as usize, expanded_key);
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
        let ik_sch: Vec<u8> = key_sch::get((rounds - x) as usize, expanded_key);
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
    let ik_sch: Vec<u8> = key_sch::get(0, expanded_key);
    // print_state(&ik_sch);
    
    // print!("\n{} - ik_add", 0);
    state = add_round_key::xor(state, ik_sch);        
    // print_state(&state);

    state
}