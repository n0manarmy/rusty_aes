extern crate rusty_aes;
extern crate rand;

use crate::rand::prelude::*;
use crate::rusty_aes::encrypt::Encrypt;
use crate::rusty_aes::decrypt::Decrypt;
use crate::rusty_aes::utils::{iv_builder, printer::print_hex_aligned};

#[test]
fn test_encrypt_decrypt_cycling() {
    println!("test_encrypt_decrypt_cycling");
    let mut run_count = 0;
    let buffer: Vec<u8> = iv_builder::get_random_bits(10000);
    let mut rando = thread_rng();

    loop {

        let key: Vec<u8> = iv_builder::get_random_bits(16);
        let o_input: Vec<u8> = iv_builder::fill_with_random_bits(&buffer, rando.gen_range(16, 10000));
        let e: Encrypt = Encrypt::ecb(key.clone());
        let cipher_text = e.encrypt(&o_input);
        
        let d: Decrypt = Decrypt::ecb(key);
        let results = d.decrypt(cipher_text);

        if o_input == results {
            run_count += 1;
        } else {
            println!("Decrypt failed");
            println!("original input:");
            print_hex_aligned(&o_input);
            println!(); 
            println!("results:");
            print_hex_aligned(&results);
            break;
        }

        if run_count % 1000 == 0 {
            println!("Successful runs: {}", run_count);
        }
    }
    
}