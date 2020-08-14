use crate::key_expander::expander;
use crate::modes;
use crate::aes_mode::AesMode;
use crate::utils::iv_builder;

pub struct Encrypt {
    pub expanded_key: Vec<u8>,
    pub rounds: u32,
    pub mode: AesMode,
    pub block_size: usize,
}

impl Encrypt {

    pub fn new(key: Vec<u8>, mode: AesMode) -> Encrypt {
        Encrypt {
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
        modes::ecb_encrypt::encrypt(&self, input)
    }

    pub fn start_cbc(self, input: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
        let iv = iv_builder::get_iv(self.block_size);
        assert_eq!(iv.len(), self.block_size);
        let results = modes::cbc_encrypt::run(&self, input, &iv);
        (results, iv)
    }
}

#[cfg(test)]
mod tests {

use super::*;
use crate::aes_mode::AesMode;
use crate::utils::{hex_encoders, printer};

    #[test]
    pub fn test_cbc_encrypt() {
        let input: Vec<u8> = "This is a test of the ability to encrypt and then decrypt the message".as_bytes().to_vec();
        let key: Vec<u8> = "YELLOW SUBMARINE".as_bytes().to_vec();
        let encryptor: Encrypt = Encrypt::new(key, AesMode::CBC);

        let (results, iv) = encryptor.start_cbc(input);
        // dbg!(results);
        // dbg!(iv);
        printer::print_hex_aligned(&results);
        printer::print_hex_aligned(&iv);


    }
}