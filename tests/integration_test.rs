extern crate rusty_aes;
extern crate rand;

use std::thread;

use crate::rand::prelude::*;
use std::io::prelude::*;
use crate::rusty_aes::encrypt::{Encrypt, InitializationValue};
use crate::rusty_aes::decrypt::Decrypt;
use crate::rusty_aes::utils::{iv_builder, printer::*};

#[test]
fn test_ecb_decrypt_openssl_file() {
    let file_name = std::path::Path::new("README.enc");
    let mut f = std::fs::File::open(file_name).expect("Error reading file");
    let mut buffer: Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer).expect("Error reading file to end");

    let key = "YELLOW_SUBMARINE";
    let d: Decrypt = Decrypt::ecb(key.as_bytes().to_vec());
    let results = d.decrypt(buffer);
    // println!("{}", print_vec(&results));

}

#[test]
fn test_cbc_encrypt_32_bit_key() {
    let message: Vec<u8> = "This is a test message that will be encrypted. The message is encrypted by rusty aes.".as_bytes().to_vec();
    let key: Vec<u8> = "THISISA32BYTEKEYWEWUSEFORENCRYPT".as_bytes().to_vec();
    let mut e: Encrypt = Encrypt::cbc(key.clone(), InitializationValue::None);
    let cipher_text = e.encrypt(&message);

    let d: Decrypt = Decrypt::cbc(key, e.get_iv());
    let results = d.decrypt(cipher_text);

    // println!("{}", print_vec(&results));
    assert_eq!(message, results);
}

#[test]
fn test_cbc_encrypt_24_bit_key() {
    let message: Vec<u8> = "This is a test message that will be encrypted. The message is encrypted by rusty aes.".as_bytes().to_vec();
    let key: Vec<u8> = "THISISTHE24BYTEKEYWEWUSE".as_bytes().to_vec();
    let mut e: Encrypt = Encrypt::cbc(key.clone(), InitializationValue::None);
    let cipher_text = e.encrypt(&message);

    let d: Decrypt = Decrypt::cbc(key, e.get_iv());
    let results = d.decrypt(cipher_text);

    // println!("{}", print_vec(&results));
    assert_eq!(message, results);
}

#[test]
fn test_cbc_encrypt_16_bit_key() {
    let message: Vec<u8> = "This is a test message that will be encrypted. The message is encrypted by rusty aes.".as_bytes().to_vec();
    let key: Vec<u8> = "THISISA16BYTEKEY".as_bytes().to_vec();
    let mut e: Encrypt = Encrypt::cbc(key.clone(), InitializationValue::None);
    let cipher_text = e.encrypt(&message);

    let d: Decrypt = Decrypt::cbc(key, e.get_iv());
    let results = d.decrypt(cipher_text);

    // println!("{}", print_vec(&results));
    assert_eq!(message, results);
}


#[test]
fn test_ecb_encrypt_32_bit_key() {
    let message: Vec<u8> = "This is a test message that will be encrypted. The message is encrypted by rusty aes.".as_bytes().to_vec();
    let key: Vec<u8> = "THISISA32BYTEKEYWEWUSEFORENCRYPT".as_bytes().to_vec();
    let mut e: Encrypt = Encrypt::ecb(key.clone());
    let cipher_text = e.encrypt(&message);

    let d: Decrypt = Decrypt::ecb(key);
    let results = d.decrypt(cipher_text);

    // println!("{}", print_vec(&results));
    assert_eq!(message, results);
}

#[test]
fn test_ecb_encrypt_24_bit_key_with_single() {
    let message: Vec<u8> = "This".as_bytes().to_vec();
    let key: Vec<u8> = "THISISTHE24BYTEKEYWEWUSE".as_bytes().to_vec();
    let mut e: Encrypt = Encrypt::ecb(key.clone());
    let cipher_text = e.encrypt(&message);

    let d: Decrypt = Decrypt::ecb(key);
    let results = d.decrypt(cipher_text);

    // println!("{}", print_vec(&results));
    assert_eq!(message, results);
}

#[test]
fn test_ecb_encrypt_24_bit_key() {
    let message: Vec<u8> = "This is a test message that will be encrypted. The message is encrypted by rusty aes.".as_bytes().to_vec();
    let key: Vec<u8> = "THISISTHE24BYTEKEYWEWUSE".as_bytes().to_vec();
    let mut e: Encrypt = Encrypt::ecb(key.clone());
    let cipher_text = e.encrypt(&message);

    let d: Decrypt = Decrypt::ecb(key);
    let results = d.decrypt(cipher_text);

    // println!("{}", print_vec(&results));
    assert_eq!(message, results);
}

#[test]
fn test_ecb_encrypt_16_bit_key() {
    let message: Vec<u8> = "This is a test message that will be encrypted. The message is encrypted by rusty aes.".as_bytes().to_vec();
    let key: Vec<u8> = "THISISA16BYTEKEY".as_bytes().to_vec();
    let mut e: Encrypt = Encrypt::ecb(key.clone());
    let cipher_text = e.encrypt(&message);

    let d: Decrypt = Decrypt::ecb(key);
    let results = d.decrypt(cipher_text);

    // println!("{}", print_vec(&results));
    assert_eq!(message, results);
}

#[test]
fn test_ecb_encrypt_decrypt_cycling() {
    // println!("test_encrypt_decrypt_cycling");
    let mut run_count = 0;
    let buffer: Vec<u8> = iv_builder::get_random_bites(10000);
    let mut rando = thread_rng();

    loop {

        let key: Vec<u8> = iv_builder::get_random_bites(16);
        let o_input: Vec<u8> = iv_builder::fill_with_random_bites(&buffer, rando.gen_range(1, 500));
        let mut e: Encrypt = Encrypt::ecb(key.clone());

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

        if run_count % 100 == 0 {
            println!("Successful runs: {}", run_count);
        }
        if run_count == 10000 {
            break;
        }
    }
    
}