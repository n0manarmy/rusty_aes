pub fn get(round: usize, exp_key: &Vec<u8>) -> Vec<u8> {
    let e_pos = round * 16;
    let mut this_exp_key: Vec<u8> = vec![0;16];
    this_exp_key.clone_from_slice(&exp_key[e_pos..e_pos + 16]);

    // let this_exp_key = transform_state(this_exp_key);

    this_exp_key
}

#[cfg(test)]
mod tests {

    use super::*;
}