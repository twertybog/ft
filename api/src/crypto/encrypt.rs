use std::error::Error;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};

pub fn enc_data(key: [u8;32], data: Vec<u8>, nonce: [u8;12]) -> Result<Vec<u8>, Box<dyn Error>>{
    let cipher = Aes256Gcm::new_from_slice(&key)?;

    let nonce = Nonce::from_slice(&nonce);
    
    let ciphertext = cipher.encrypt(nonce, data.as_ref())
        .expect("Data not encrypted!");

    Ok(ciphertext)
}