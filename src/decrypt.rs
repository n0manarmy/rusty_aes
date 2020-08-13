use crate::key_expander::expander;

pub struct Decrypt {
    pub expanded_key: Vec<u8>,
    pub rounds: u32,
}

impl Decrypt {

    pub fn new(key: Vec<u8>) -> Decrypt {
        Decrypt {
            expanded_key: expander::expand(&key),
            rounds: Self::get_rounds(key.len()),
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
}