pub fn collect_to_vec(word: u32) -> Vec<u8> {
    let mut val: Vec<u8> = Vec::new();
    val.push(((word & 0xFF000000) >> 24) as u8);
    val.push(((word & 0xFF0000) >> 16) as u8);
    val.push(((word & 0xFF00) >> 8) as u8);
    val.push((word & 0xFF) as u8);

    val

}

pub fn rot_word(mut word: u32) -> u32{
    let mut f_u8 = 0xFF000000 & word;
    f_u8 >>= 24;
    word <<= 8;
    word |= f_u8;

    word
}


pub fn rcon(rounds: usize, key_size: usize) -> u32 {
    let lookup = (rounds/(key_size/4)) - 1;
    match lookup {
        0   => 0x01000000, 
        1   => 0x02000000, 
        2   => 0x04000000, 
        3   => 0x08000000, 
        4   => 0x10000000, 
        5   => 0x20000000, 
        6   => 0x40000000, 
        7   => 0x80000000, 
        8   => 0x1B000000, 
        9   => 0x36000000, 
        10  => 0x6C000000, 
        11  => 0xD8000000, 
        12  => 0xAB000000, 
        13  => 0x4D000000, 
        14  => 0x9A000000,
        _   => panic!("Error in rcon lookup table"),
    }

}

// we collect 4 bytes from the expanded key into a u32 for manipulation
pub fn ek(offset: usize, key: &Vec<u8>) -> u32 {
    let mut word: u32 = 0;
    for x in offset..offset + 4 {
        word |= key[x] as u32;
        if x == offset + 3 {
            break;
        }
        word <<= 8;
    }

    word
}

pub fn k(offset: usize, key: &Vec<u8>) -> Vec<u8> {
    let mut val: Vec<u8> = Vec::new();
    for x in offset..offset + 4 {
        val.push(key[x]);
    }

    val
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::encrypt_funcs;

    #[test]
    pub fn test_collect_to_vec() {
        let val: u32 = 0xf2c295f2;
        let result: Vec<u8> = vec![0xf2, 0xc2, 0x95, 0xf2];
        assert_eq!(collect_to_vec(val), result);
    }

    #[test]
    pub fn test_process() {
        let val: u32 = 0x2a6c7605;
        let rotted: u32 = rot_word(val);
        assert_eq!(rotted, 0x6c76052a);
        let subbed: u32 = encrypt_funcs::byte_sub::byte_sub(rotted);
        assert_eq!(subbed, 0x50386be5);
        let rconned: u32 = rcon(8, 16);
        assert_eq!(rconned, 0x02000000);

    }

    #[test]
    pub fn test_ek() {
        let encrypt_key: Vec<u8> = vec![0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
        let result: u32 = 0x09cf4f3c;
        let key: u32 = ek((4-1) * 4, &encrypt_key);
        // println!("result: {:02x}", result);
        // println!("key: {:02x}", key);
        assert_eq!(key, result)

    }

    #[test]
    pub fn test_k() {
        let key: Vec<u8> = vec![1,2,3,4];
        dbg!(k(0, &key));
    }

    #[test]
    pub fn test_sub_word() {
        let val: u32 = 0x19;
        let result = encrypt_funcs::byte_sub::byte_sub(val);
        dbg!(result);
        assert_eq!(result, 0x636363d4);

    }

    #[test]
    pub fn test_rot_word() {
        // let key: Vec<u8> = vec![10,20,30,40];
        // let result: Vec<u8> = vec![20,30,40,10];
        // let key: u32 = 0xffffff00;
        // let result: u32 = 0xffffffff;
        // let key = rot_word(key);
        // assert_eq!(key, result);

        let key: u32 = 0x09CF4F3C;
        let result: u32 = 0xCF4F3C09;
        let key = rot_word(key);
        // println!("{:02x}", key);
        assert_eq!(key, result);
    }

    #[test]
    pub fn test_rcon() {
        let rounds = 4;
        let key_size = 16;
        assert_eq!(rcon(rounds, key_size), 0x01000000);
    }
}