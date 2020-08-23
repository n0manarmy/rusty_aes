use crate::utils::printer::*;

/// Padder takes len and buf and appends to fill buffer to correct length
/// 
/// # Arguments
/// 
/// `val`       - the current buffer
/// `pad_len`   - The len to pad
/// 
pub fn pad(mut val: Vec<u8>, block_size: usize) -> Vec<u8> {
    let pad_len = block_size - val.len();
    if pad_len == 0 {
        return val;
    }
    //we pad 17 17x, because if we pad 1, there could be a 1 value. 
    else if pad_len == 1 {
        for _x in 0..17 {
            val.push(17 as u8)
        }
        // print!("single pad vaue required: ");
        // print_hex_aligned(&val);
        return val;
    } else {
        for _x in 0..pad_len {
            val.push(pad_len as u8);
        }
    
        return val;
    }
}

pub fn clear_padding(mut buf: Vec<u8>) -> Vec<u8> {
    // print!("clear_padding enter: ");
    // print_hex_aligned(&buf);
    let pad_val = buf[buf.len() - 1];
    if pad_val > 17 {
        return buf;
    } else if pad_val > 1 {
        let mut pos = buf.len() - 1;
        let mut pad_val_count = pad_val;
        while pad_val_count > 0 {
            if buf[pos] != pad_val {
                return buf;
            } 
            pos -= 1;
            pad_val_count -= 1;
        }

        let mut pos = buf.len() - 1;
        let mut pad_val_count = pad_val;

        while pad_val_count > 0 {
            buf.remove(pos);
            pos -= 1;
            pad_val_count -= 1;
        }   
    }

    buf
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::printer::*;

    #[test]
    pub fn test_pad() {
        let val: Vec<u8> = "test".as_bytes().to_vec();
        let val = pad(val, 16);
        print_hex_aligned(&val);
        assert_eq!(val[15], 12 as u8);
        let val: Vec<u8> = "testtest".as_bytes().to_vec();
        let val = pad(val, 16);
        print_hex_aligned(&val);
        assert_eq!(val[15], 8 as u8);
        let val: Vec<u8> = "testtesttest".as_bytes().to_vec();
        let val = pad(val, 16);
        print_hex_aligned(&val);
        assert_eq!(val[15], 4 as u8);
        let val: Vec<u8> = "testtesttesttes".as_bytes().to_vec();
        let val = pad(val, 16);
        print_hex_aligned(&val);
        assert_eq!(val[15], 17);
        let val: Vec<u8> = "testtesttestte".as_bytes().to_vec();
        let val = pad(val, 16);
        print_hex_aligned(&val);
        assert_eq!(val[15], 2);
    }
}