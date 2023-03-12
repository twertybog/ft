use aes::Aes256;
use std::error::Error;
use aes::cipher::{
    BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
pub fn dec_data(key: [u8;32], data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>{
    let cipher = Aes256::new_from_slice(&key)?;
    
    let mut block = *GenericArray::from_slice(&data);

    cipher.decrypt_block(&mut block);

    let mut decrypt_data = block.to_vec();

    for i in (0..decrypt_data.len()).rev(){
        if decrypt_data[i] != 0{
            break;
        }
        else{
            decrypt_data.remove(i);
        }
    }
    Ok(decrypt_data)
}