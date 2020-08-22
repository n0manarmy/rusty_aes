/// Padder takes len and buf and appends to fill buffer to correct length
/// 
/// # Arguments
/// 
/// `val`       - the current buffer
/// `pad_len`   - The len to pad
/// 
pub fn pad(mut val: Vec<u8>, pad_len: usize) -> Vec<u8> {
    dbg!(val.len());
    dbg!(pad_len);
    let padding = 0x80;
    let pad_len = pad_len - val.len();
    for _x in 0..pad_len {
        val.push(padding);
    }

    dbg!(pad_len);

    val.push(pad_len as u8);

    val
}

pub fn clear_padding(mut buf: Vec<u8>) -> Vec<u8> {
    if buf[buf.len() - 2] == 0x80 {
        let mut pad_len = buf[buf.len() - 1];
        dbg!(pad_len);
        while pad_len > 0 {
            buf.pop();
            pad_len -= 1;
        }   
    }
    buf
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_pad() {
        let val: Vec<u8> = "test".as_bytes().to_vec();
        let val = pad(val, 16);
        assert_eq!(val[15], 12 as u8);
        dbg!(val);
    }
}