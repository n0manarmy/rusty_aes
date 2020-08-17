use crate::key_expander::expander;
use crate::modes;
use crate::aes_mode::AesMode;
use crate::utils::iv_builder;

pub enum InitializationValue {
    IV(Vec<u8>),
    None,
}

pub struct Encrypt {
    pub expanded_key: Vec<u8>,
    pub rounds: u32,
    pub mode: AesMode,
    pub block_size: usize,
    pub iv: InitializationValue,
}

impl Encrypt {

    pub fn new(key: Vec<u8>, mode: AesMode, iv: InitializationValue) -> Encrypt {
        Encrypt {
            expanded_key: expander::expand(&key),
            rounds: Self::get_rounds(key.len()),
            mode,
            block_size: key.len(),
            iv,
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
        modes::ecb_encrypt::run(self, input)
    }

    pub fn start_cbc(mut self, input: Vec<u8>) -> Vec<u8> {
        let iv = match self.iv {
            InitializationValue::IV(v) => v,
            InitializationValue::None => iv_builder::get_iv(self.block_size),
        };
        self.iv = InitializationValue::IV(iv.clone());
        let results = modes::cbc_encrypt::run(&self, input, iv);

        results
    }
}

#[cfg(test)]
mod tests {

use super::*;
use crate::aes_mode::AesMode;
use crate::utils::{hex_encoders, printer};

    #[test]
    pub fn ietf_cbc_128_encrypt_test() {
        let key: Vec<u8> = vec![0x06, 0xa9, 0x21, 0x40, 0x36, 0xb8, 0xa1, 0x5b, 0x51, 0x2e, 0x03, 0xd5, 0x34, 0x12, 0x00, 0x06];
        let iv = InitializationValue::IV(vec![0x3d, 0xaf, 0xba, 0x42, 0x9d, 0x9e, 0xb4, 0x30, 0xb4, 0x22, 0xda, 0x80, 0x2c, 0x9f, 0xac, 0x41]);
        let input = "Single block msg".as_bytes().to_vec();
        let cipher_answer: Vec<u8> = vec![0xe3, 0x53, 0x77, 0x9c, 0x10, 0x79, 0xae, 0xb8, 0x27, 0x08, 0x94, 0x2d, 0xbe, 0x77, 0x18, 0x1a];

        let encryptor: Encrypt = Encrypt::new(key, AesMode::CBC, iv);
        let results = encryptor.start_cbc(input);
        assert_eq!(results, cipher_answer);
        printer::print_hex_aligned(&results);



    }

    #[test]
    pub fn test_ecb_encrypt() {
        let input: Vec<u8> = "This is a test of the ability to encrypt and then decrypt the message".as_bytes().to_vec();
        let key: Vec<u8> = "YELLOW SUBMARINE".as_bytes().to_vec();
        let encryptor: Encrypt = Encrypt::new(key, AesMode::ECB, InitializationValue::None);

        let results = encryptor.start_ecb(input);
        // dbg!(results);
        // dbg!(iv);
        printer::print_hex_aligned(&results);
    }

    #[test]
    pub fn test_cbc_encrypt() {
        let input: Vec<u8> = "This is a test of the ability to encrypt and then decrypt the message".as_bytes().to_vec();
        let key: Vec<u8> = "YELLOW SUBMARINE".as_bytes().to_vec();
        let encryptor: Encrypt = Encrypt::new(key, AesMode::CBC, InitializationValue::None);

        let results = encryptor.start_cbc(input);
        // dbg!(results);
        // dbg!(iv);
        printer::print_hex_aligned(&results);


    }
}