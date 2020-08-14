pub fn pad(mut val: Vec<u8>, pad_len: usize) -> Vec<u8> {
    // let padding = 0x04;
    let padding = 0x80;
    for _x in 0..(pad_len - val.len()) {
        val.push(padding);
    }

    val
}