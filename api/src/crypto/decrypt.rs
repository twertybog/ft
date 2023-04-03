use aes_gcm::{
    Error,
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};

pub fn dec_data(key: [u8;32], data: Vec<u8>, nonce: [u8; 12]) -> Result<Vec<u8>, Error>{
    let cipher = Aes256Gcm::new_from_slice(&key)
        .expect("Key is invalid!");

    let nonce = Nonce::from_slice(&nonce);
    
    let plaintext = match cipher.decrypt(nonce, data.as_ref()){
        Ok(data) => data,
        Err(err) => return Err(err),
    };
    
    Ok(plaintext)
}