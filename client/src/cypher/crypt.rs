use std::error::Error;
use aes::Aes256;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

pub fn enc_data(key: [u8;32], mut data: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>>{
    let cipher = Aes256::new_from_slice(&key)?;

    while data.len() < 16 {
        data.push(0);
    }

    let mut block = *GenericArray::from_slice(&data);

    cipher.encrypt_block(&mut block);

    Ok(block.to_vec())
}