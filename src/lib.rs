// use log::{info};

extern crate rand;

pub mod decrypt;
pub mod encrypt;
pub mod utils;
pub mod aes_mode;
pub mod logger;
mod modes;
mod key_expander;
mod test_vals;
mod encrypt_funcs;
mod decrypt_funcs;

pub mod prelude {
    pub use crate::decrypt::*;
    pub use crate::encrypt::*;
    pub use crate::utils::*;
    pub use crate::utils::helper::*;
    pub use crate::utils::tables::*;
    pub use crate::aes_mode::*;
    pub use crate::logger::*;
    pub use crate::key_expander::*;
    pub use crate::key_expander::helper::*;
    pub use crate::key_expander::expander::*;
    pub use crate::encrypt_funcs::*;
    pub use crate::decrypt_funcs::*;
    pub use crate::modes::*;
    pub use crate::logger::build_logger::*;
}

// use crate::logger::build_logger::{LogBuilder, LogLevel};

// fn main() {
//     LogBuilder::build_logger(LogLevel::Debug);
//     info!("Logger started");

// }