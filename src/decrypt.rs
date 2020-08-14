use crate::key_expander::expander;
use crate::modes;
use crate::aes_mode::AesMode;

pub struct Decrypt {
    pub expanded_key: Vec<u8>,
    pub rounds: u32,
    pub mode: AesMode,
    pub block_size: usize,
}

impl Decrypt {

    pub fn new(key: Vec<u8>, mode: AesMode) -> Decrypt {
        Decrypt {
            expanded_key: expander::expand(&key),
            rounds: Self::get_rounds(key.len()),
            mode,
            block_size: key.len(),
        }
    }

    fn get_rounds(key_len: usize) -> u32 {
        match key_len {
            16 => 10,
            24 => 12,
            32 => 14,
            _  => panic!("unexpended key size found"),
        }
    }

    pub fn start_ecb(self, input: Vec<u8>) -> Vec<u8> {
        modes::ecb_decrypt::run(self, input)
    }

    pub fn start_cbc(self, input: Vec<u8>, iv: Vec<u8>) -> Vec<u8> {
        modes::cbc_decrypt::run(&self, input, iv)
    }
}

#[cfg(test)]
mod tests {

use super::*;
use crate::aes_mode::AesMode;
use crate::utils::{hex_encoders, printer};

    #[test]
    pub fn test_ecb_decrypt() {
        // let input: Vec<u8> = "This is a test of the ability to encrypt and then decrypt the message".as_bytes().to_vec();
        let input: Vec<u8> = hex_encoders::str_to_hex_u8_buf("f6d6bba9f488c9e2bda504273828112f7d9fc76fe885250877ecbe77b019d10c6bae36c20d012c7821e01caf7e6b21862279c04d2ca230df2062fbc235a2afad929b25807e924f93db965c7ed258b1ed");
        let key: Vec<u8> = "YELLOW SUBMARINE".as_bytes().to_vec();
        let decrypt: Decrypt = Decrypt::new(key, AesMode::ECB);

        let results = decrypt.start_ecb(input);
        // dbg!(results);
        // dbg!(iv);
        // printer::print_hex_aligned(&results);
        for r in results {
            print!("{}", r as char);
        }
        println!();
    }


    #[test]
    pub fn test_cbc_decrypt() {
        // let input: Vec<u8> = "This is a test of the ability to encrypt and then decrypt the message".as_bytes().to_vec();
        let input: Vec<u8> = hex_encoders::str_to_hex_u8_buf("81dddc8f45635f3c8113bda618af39a348b93b1cc3ea75e8f066f91ba70aaf54f37c3835ccf686665934a09f17219e6b6e5e4cfe277b881275987c46e1f822c820f85a9630fa9bfc0cc5e782c199cfb1");
        let key: Vec<u8> = "YELLOW SUBMARINE".as_bytes().to_vec();
        let iv: Vec<u8> = hex_encoders::str_to_hex_u8_buf("3bd9688be939895e463491759d30a92d");
        let decrypt: Decrypt = Decrypt::new(key, AesMode::CBC);

        let results = decrypt.start_cbc(input, iv);
        // dbg!(results);
        // dbg!(iv);
        // printer::print_hex_aligned(&results);
        for r in results {
            print!("{}", r as char);
        }
        println!();
    }
}