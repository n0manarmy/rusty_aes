use crate::key_expander::expander;
use crate::modes;
use crate::aes_mode::AesMode;

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

    pub fn start_mode(self, input: Vec<u8>) -> Vec<u8> {
        match self.mode {
            AesMode::CBC => modes::cbc_encrypt::encrypt(&self, input),
            AesMode::ECB => modes::ecb_encrypt::encrypt(&self, input),
        }
    }
}