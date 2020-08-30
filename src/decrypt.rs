use crate::key_expander::expander;
use crate::modes;
use crate::aes_mode::AesMode;

pub struct Decrypt {
    pub expanded_key: Vec<u8>,
    pub iv: Vec<u8>,
    pub rounds: u32,
    pub mode: AesMode,
    pub block_size: usize,
}

impl Decrypt {

    fn get_rounds(key_len: usize) -> u32 {
        match key_len {
            16 => 10,
            24 => 12,
            32 => 14,
            _  => panic!("unexpended key size found"),
        }
    }

    pub fn ecb(key: Vec<u8>) -> Decrypt {
        Decrypt {
            expanded_key: expander::expand(&key),
            rounds: Self::get_rounds(key.len()),
            block_size: 16,
            iv: Vec::new(),
            mode: AesMode::ECB,
        }
    }

    pub fn cbc(key: Vec<u8>, iv: Vec<u8>) -> Decrypt {
        Decrypt {
            expanded_key: expander::expand(&key),
            rounds: Self::get_rounds(key.len()),
            iv,
            block_size: 16,
            mode: AesMode::CBC,
        }
    }
    
    pub fn decrypt(self, input: Vec<u8>) -> Vec<u8> {
        match self.mode {
            AesMode::ECB => modes::ecb_decrypt::run(self, input),
            AesMode::CBC => modes::cbc_decrypt::run(&self, input),
        }
    }
}

#[cfg(test)]
mod tests {

use super::*;
use crate::utils::{hex_encoders, printer};


    #[test]
    pub fn ietf_cbc_128_decrypt_test() {
        let key: Vec<u8> = vec![0x06, 0xa9, 0x21, 0x40, 0x36, 0xb8, 0xa1, 0x5b, 0x51, 0x2e, 0x03, 0xd5, 0x34, 0x12, 0x00, 0x06];
        let iv: Vec<u8> = vec![0x3d, 0xaf, 0xba, 0x42, 0x9d, 0x9e, 0xb4, 0x30, 0xb4, 0x22, 0xda, 0x80, 0x2c, 0x9f, 0xac, 0x41];
        let input = "Single block msg".as_bytes().to_vec();
        let cipher_answer: Vec<u8> = vec![0xe3, 0x53, 0x77, 0x9c, 0x10, 0x79, 0xae, 0xb8, 0x27, 0x08, 0x94, 0x2d, 0xbe, 0x77, 0x18, 0x1a];

        let decryptor: Decrypt = Decrypt::cbc(key, iv);
        let results = decryptor.decrypt(cipher_answer);
        assert_eq!(results, input);
        // printer::print_hex_aligned(&results);
    }


    #[test]
    pub fn test_ecb_decrypt() {
        // let input: Vec<u8> = "This is a test of the ability to encrypt and then decrypt the message".as_bytes().to_vec();
        let input: Vec<u8> = hex_encoders::str_to_hex_u8_buf("f6d6bba9f488c9e2bda504273828112f7d9fc76fe885250877ecbe77b019d10c6bae36c20d012c7821e01caf7e6b21862279c04d2ca230df2062fbc235a2afad929b25807e924f93db965c7ed258b1ed");
        let key: Vec<u8> = "YELLOW SUBMARINE".as_bytes().to_vec();
        let decrypt: Decrypt = Decrypt::ecb(key);

        let results = decrypt.decrypt(input);
        // dbg!(results);
        // dbg!(iv);
        // printer::print_hex_aligned(&results);
        // for r in results {
        //     print!("{}", r as char);
        // }
        // println!();
    }


    #[test]
    pub fn test_cbc_decrypt() {
        // let input: Vec<u8> = "This is a test of the ability to encrypt and then decrypt the message".as_bytes().to_vec();
        let input: Vec<u8> = hex_encoders::str_to_hex_u8_buf("81dddc8f45635f3c8113bda618af39a348b93b1cc3ea75e8f066f91ba70aaf54f37c3835ccf686665934a09f17219e6b6e5e4cfe277b881275987c46e1f822c820f85a9630fa9bfc0cc5e782c199cfb1");
        let key: Vec<u8> = "YELLOW SUBMARINE".as_bytes().to_vec();
        let iv: Vec<u8> = hex_encoders::str_to_hex_u8_buf("3bd9688be939895e463491759d30a92d");
        let decrypt: Decrypt = Decrypt::cbc(key, iv);

        let results = decrypt.decrypt(input);
        // dbg!(results);
        // dbg!(iv);
        // printer::print_hex_aligned(&results);
        // for r in results {
        //     print!("{}", r as char);
        // }
        // println!();
    }
}