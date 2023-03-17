use std::error::Error;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use crate::del_zeros;

pub fn dec_data(key: [u8;32], mut data: Vec<u8>, nonce: [u8; 12]) -> Result<Vec<u8>, Box<dyn Error>>{
    let cipher = Aes256Gcm::new_from_slice(&key)?;

    let nonce = Nonce::from_slice(&nonce);

    del_zeros(&mut data);

    let plaintext = cipher.decrypt(nonce, data.as_ref())
        .expect("Data not decrypted!");

    Ok(plaintext)
}