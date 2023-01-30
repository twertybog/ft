use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes256;
use f2b::{b2f, f2b_part};
use k256::{ecdh::EphemeralSecret, EncodedPoint, PublicKey};
use rand::rngs::ThreadRng;
use rand::Rng;
use rand_core::OsRng;
use std::{error, fs, io};
use tokio::{
    io::{split, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:4956").await?;

    loop {
        let (socket, address) = listener.accept().await?;

        let (mut rd, mut wd) = split(socket);

        let get_secret = tokio::spawn(async move {
            //create server keys
            let server_secret = EphemeralSecret::random(OsRng);

            let server_public = EncodedPoint::from(server_secret.public_key());

            //get client pub key
            let mut client_public = Vec::new();

            rd.read_buf(&mut client_public).await
                .expect("Client public key not received!");

            let client_public = PublicKey::from_sec1_bytes(client_public.as_ref())
                .expect("Client public key is invalid");

            //share pub key
            wd.write_all(server_public.as_ref()).await
                .expect("Server public key not sent!");

            server_secret.diffie_hellman(&client_public)
        });

        println!("{:?}\n", get_secret.await?.raw_secret_bytes());
    }
}

fn key_generation(rng: &mut ThreadRng) -> Vec<u8> {
    let mut key: Vec<u8> = Vec::new();

    for _ in 0..32 {
        key.push(rng.gen_range(0..=255));
    }
    key
}

fn enc_data(cipher: &Aes256, path: &str) {
    let mut counter: u64 = 0;

    let file_size = fs::metadata(path).unwrap().len();

    let file_result = fs::File::create("encdata").unwrap();

    while counter < file_size {
        let mut data = f2b_part(path, counter, 16).unwrap();

        if data.len() != 16 {
            for _ in data.len()..16 {
                data.push(0);
            }
        }

        let mut block = *GenericArray::from_slice(&data[..]);

        cipher.encrypt_block(&mut block);

        b2f(&block.to_vec(), &file_result).unwrap();

        counter += 16;
    }
}

fn dec_data(cipher: &Aes256, path: &str) {
    let mut counter: u64 = 0;

    let file_size = fs::metadata(path).unwrap().len();

    let file_result = fs::File::create("decdata.toml").unwrap();

    while counter < file_size {
        let data = f2b_part(path, counter, 16).unwrap();

        let mut block = *GenericArray::from_slice(&data[..]);

        cipher.decrypt_block(&mut block);

        b2f(&block.to_vec(), &file_result);

        counter += 16;
    }
}
