use std::fs::File;
use std::io::prelude::*;

pub fn read_random(size: usize) -> Vec<u8>{
    let mut buf_vec: Vec<u8> = Vec::with_capacity(size);
    if cfg!(unix) {
        let path = "/dev/random";
        let mut file_reader = match File::open(path) {
            Ok(k) => k,
            Err(why) => panic!(why),
        };
        match file_reader.read_exact(&mut buf_vec) {
            Ok(b) => b,
            Err(why) => panic!(why),
        };
    }

    buf_vec
}