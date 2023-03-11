mod secret;
mod crypt;
mod decrypt;
pub use secret::get_secret;
pub use crypt::enc_data;
pub use decrypt::dec_data;