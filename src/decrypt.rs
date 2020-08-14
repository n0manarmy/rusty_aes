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
        modes::ecb_decrypt::decrypt(&self, input)
    }

    pub fn start_cbc(self, input: Vec<u8>, iv: Vec<u8>) -> Vec<u8> {
        modes::cbc_decrypt::run(&self, input, &iv)
    }
}

#[cfg(test)]
mod tests {

use super::*;
use crate::aes_mode::AesMode;
use crate::utils::{hex_encoders, printer};

    #[test]
    pub fn test_cbc_decrypt() {
        // let input: Vec<u8> = "This is a test of the ability to encrypt and then decrypt the message".as_bytes().to_vec();
        let input: Vec<u8> = hex_encoders::str_to_hex_u8_buf("6974a61583f424f4dac8ff4dd922d4904bd7b38b1c1c27a70f093b0d05a7eda0b1efc4989a80736eff39e97419469123e26ff42c30ec3d7a3d0646e060ba0ceedc175eb774269162f314ffb8eb7b03a1");
        let key: Vec<u8> = "YELLOW SUBMARINE".as_bytes().to_vec();
        let iv: Vec<u8> = hex_encoders::str_to_hex_u8_buf("1c847943e4a09dd9c9fc5c673246ea35");
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