pub fn str_to_hex_val(buf: String) -> Vec<u32> {
    let mut hex_buf: Vec<u32> = Vec::new();
    
    let mut hex_merge: u32 = 0;
    let mut pos = 1;
    
    for c in buf.chars() {
        hex_merge |= c.to_digit(16).unwrap();

        if pos == 2 {
            hex_buf.push(hex_merge);
            hex_merge = 0;
            pos = 1;
            continue;
        }

        pos += 1;
        hex_merge <<= 4;
    }
    
    hex_buf
}

pub fn ascii_to_ascii_hex(buf: &str) -> String {
    let mut r: String = String::new();
    use std::fmt::Write as FmtWrite;
    for b in buf.as_bytes() {
        write!(r, "{:02x}", b);
    }

    r
}

pub fn str_to_hex_u8_buf(buf: &str) -> Vec<u8> {
    let mut hex_buf: Vec<u8> = Vec::new();
    
    let mut hex_merge: u8 = 0;
    let mut pos = 1;
    
    for c in buf.chars() {
        hex_merge |= translate_char_to_hex_val(c) as u8;

        if pos == 2 {
            hex_buf.push(hex_merge);
            hex_merge = 0;
            pos = 1;
            continue;
        }

        pos += 1;
        hex_merge <<= 4;
    }
    
    hex_buf
}

pub fn translate_char_to_hex_val(x: char) -> usize {
    match x.to_ascii_lowercase() {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _   => panic!("Error in translating hex val to usize val"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_str_to_hex_u8() {
        let input = "00112233445566778899aabbccddeeff";
        let input: Vec<u8> = str_to_hex_u8_buf(input);
        // for i in input {
        //     print!("{:02x} ", i);
        // }
    }

    #[test]
    pub fn test_alpha_to_hex() {
        let alpha = "abcdefghijklmnopqrstuvwxyz";
        // for a in alpha.chars() {
        //     println!("{}", a);
        // }
    }
}