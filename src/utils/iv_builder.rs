use crate::rand::prelude::*;
use crate::utils::file_helper;

pub fn get_iv(size: usize) -> Vec<u8> {
    file_helper::read_dev_random(size)
}

pub fn get_random_bites(amount: usize) -> Vec<u8> {
    file_helper::read_dev_urandom(amount)
}

pub fn fill_with_random_bites(buf: &Vec<u8>, count: usize) -> Vec<u8> {
    let mut temp: Vec<u8> = Vec::new();
    let mut rando = thread_rng();

    for _x in 0..count {
        temp.push(buf[rando.gen_range(0, buf.len() - 1)]);
    }

    temp
}