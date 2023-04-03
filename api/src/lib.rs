use rand::Rng;
mod crypto;
mod files;

pub use files::{get_file, send_file};
pub use crypto::{dec_data, enc_data, send_secret, get_secret};


pub fn gen_nonce() -> [u8;12]{
    let mut rng = rand::thread_rng();

    let mut nonce = [0;12];
    
    rng.fill(&mut nonce);

    nonce
}

pub fn del_zeros(data: &mut Vec<u8>){
    for i in (0..data.len()).rev(){
        if data[i] != 0{
            break;
        }
        else {
            data.remove(i);
        }
    }
}
