use crate::key_expander::expander;
use crate::modes;
use crate::aes_mode::AesMode;
use crate::utils::iv_builder;

/// Manages the state of the IV 
pub enum InitializationValue {
    IV(Vec<u8>),
    None,
}

/// Encrypt values required are represented here
/// 
/// # Arguments
/// 
/// `expanded_key`  - The key after its been expanded by the AES process
/// `rounds`        - The number of rounds based on the key size
/// `mode`          - The AES encryption mode (CBC, ECB)
/// `block_size`    - The size of the blocks used based on key size. Fixed 16 bytes
/// `iv`            - The initialization vector value or None.
/// 
pub struct Encrypt {
    pub expanded_key: Vec<u8>,
    pub rounds: u32,
    pub mode: AesMode,
    pub block_size: usize,
    pub iv: InitializationValue,
}

impl Encrypt {
    /// Returns an Encrypt based on AES Electronic Code Book (ECB), initialized and ready to accept an input
    /// 
    /// # Arguments
    /// 
    /// * `key` - A Vec<u8> representation of the key value. 
    /// 
    /// # Examples
    /// 
    /// ```
    /// let key: Vec<u8> = "MYSIXTEENBYTEKEY".as_bytes().to_vec();
    /// let encryptor: Encryt = Encrypt::new(key)
    /// ```
    pub fn ecb(key: Vec<u8>) -> Encrypt {
        Encrypt {
            expanded_key: expander::expand(&key),
            rounds: Self::get_rounds(key.len()),
            mode: AesMode::ECB,
            block_size: 16,
            iv: InitializationValue::None,
        }
    }

    /// Helper function to return the IV after its initialized
    /// 
    pub fn get_iv(self) -> Vec<u8> {
        match self.iv {
            InitializationValue::IV(v) => v,
            _ => panic!("No IV exists"),
        }
    }

    /// Returns an Encrypt based on AES Cipher Block Chaining (CBC), initialized and ready to accept an input
    /// 
    /// # Arguments
    /// 
    /// * `key` - A Vec<u8> representation of the key value. 
    /// * `iv`  - A IV ENUM Value if an IV is specified. If An IV is not specified then one will be (weakly) 
    ///             generated and pushed back into the self.encrypt when execution is done. This can be retrieved
    ///             by accessing the struct iv value after execution.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let key: Vec<u8> = "MYSIXTEENBYTEKEY".as_bytes().to_vec();
    /// let encryptor: Encryt = Encrypt::new(key)
    /// ```
    pub fn cbc(key: Vec<u8>, iv: InitializationValue) -> Encrypt {
        Encrypt {
            expanded_key: expander::expand(&key),
            rounds: Self::get_rounds(key.len()),
            mode: AesMode::CBC,
            block_size: 16,
            iv,
        }

    }

    /// get_rounds calculates the rounds based on the key size which is very specific to AES
    fn get_rounds(key_len: usize) -> u32 {
        match key_len {
            16 => 10,
            24 => 12,
            32 => 14,
            _  => panic!("unexpended key size found"),
        }
    }

    /// encrypt starts the encryption process
    /// 
    /// # Arguments
    /// 
    /// * `input`   - A Vec<u8> of some value to be encrypted
    /// 
    /// # Returns
    /// 
    /// * A Vec<u8> of cipher text encrypted
    /// 
    pub fn encrypt(&mut self, input: &Vec<u8>) -> Vec<u8> {
        match self.mode {
            AesMode::ECB => modes::ecb_encrypt::run(&self, &input),
            AesMode::CBC => {
                let iv: Vec<u8> = match &self.iv {
                    InitializationValue::None => iv_builder::get_iv(self.block_size),
                    InitializationValue::IV(v) => v.clone(),
                };
                self.iv = InitializationValue::IV(iv.clone());
                
                modes::cbc_encrypt::run(&self, &input, iv)
            },
        }
    }
}

#[cfg(test)]
mod tests {

use super::*;
use crate::utils::printer;

    #[test]
    pub fn ietf_cbc_128_encrypt_test() {
        let key: Vec<u8> = vec![0x06, 0xa9, 0x21, 0x40, 0x36, 0xb8, 0xa1, 0x5b, 0x51, 0x2e, 0x03, 0xd5, 0x34, 0x12, 0x00, 0x06];
        let iv = InitializationValue::IV(vec![0x3d, 0xaf, 0xba, 0x42, 0x9d, 0x9e, 0xb4, 0x30, 0xb4, 0x22, 0xda, 0x80, 0x2c, 0x9f, 0xac, 0x41]);
        let input = "Single block msg".as_bytes().to_vec();
        let cipher_answer: Vec<u8> = vec![0xe3, 0x53, 0x77, 0x9c, 0x10, 0x79, 0xae, 0xb8, 0x27, 0x08, 0x94, 0x2d, 0xbe, 0x77, 0x18, 0x1a];

        let mut encryptor: Encrypt = Encrypt::cbc(key, iv);
        let results = encryptor.encrypt(&input);
        assert_eq!(results, cipher_answer);
        printer::print_hex_aligned(&results);



    }

    #[test]
    pub fn test_ecb_encrypt() {
        let input: Vec<u8> = "This is a test of the ability to encrypt and then decrypt the message".as_bytes().to_vec();
        let key: Vec<u8> = "YELLOW SUBMARINE".as_bytes().to_vec();
        let mut encryptor: Encrypt = Encrypt::ecb(key);

        let results = encryptor.encrypt(&input);
        // dbg!(results);
        // dbg!(iv);
        printer::print_hex_aligned(&results);
    }

    #[test]
    pub fn test_cbc_encrypt() {
        let input: Vec<u8> = "This is a test of the ability to encrypt and then decrypt the message".as_bytes().to_vec();
        let key: Vec<u8> = "YELLOW SUBMARINE".as_bytes().to_vec();
        let mut encryptor: Encrypt = Encrypt::cbc(key, InitializationValue::None);

        let results = encryptor.encrypt(&input);
        // dbg!(results);
        // dbg!(iv);
        printer::print_hex_aligned(&results);


    }
}