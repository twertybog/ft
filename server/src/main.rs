#![allow(unused_imports)]
use std::sync::Arc;
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes256;
use std::path::Path;
use f2b::{b2f, f2b_part};
use k256::{ecdh::EphemeralSecret, EncodedPoint, PublicKey};
use rand_core::OsRng;
use std::{error};
use tokio::{
    sync::Mutex,
    fs::read_dir,
    io::{split, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener},
};

mod secret;
use secret::get_secret;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:4956").await?;

    loop {
        let (socket, _address) = listener.accept().await?;

        let stream = Arc::new(Mutex::new(socket));

        println!("Secret: {:?}", get_secret(stream.clone()).await);
    }
}

