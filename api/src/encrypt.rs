use std::error::Error;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};

pub fn enc_data(key: [u8;32], data: Vec<u8>, nonce: [u8;12]) -> Result<Vec<u8>, Box<dyn Error>>{
    let cipher = Aes256Gcm::new_from_slice(&key)?;
    
    let len_before = data.len();

    let nonce = Nonce::from_slice(&nonce);
    
    let mut  ciphertext = cipher.encrypt(nonce, data.as_ref())
        .expect("Data not encrypted!");

    while len_before + 16 < ciphertext.len(){
        ciphertext.push(0);
    }
    Ok(ciphertext)
}