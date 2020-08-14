use crate::utils::file_helper::read_dev_random;

pub fn get_iv(size: usize) -> Vec<u8> {
    read_dev_random(size)
}