extern crate rand;

pub mod decrypt;
pub mod encrypt;
pub mod utils;
pub mod aes_mode;
mod modes;
mod key_expander;
mod test_vals;
mod encrypt_funcs;
mod decrypt_funcs;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
