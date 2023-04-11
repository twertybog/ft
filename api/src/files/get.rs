use crate::{dec_data, del_zeros, send_secret};
use f2b::b2f;
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub async fn get_file(stream: Arc<Mutex<TcpStream>>, flag: &str) {
    let _file_len = stream
        .lock()
        .await
        .read_i64()
        .await
        .expect("Can't get file length!");

    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(flag)
        .await
        .expect("File not created!");

    let secret = send_secret(stream.clone())
        .await
        .expect("Key not generated!");

    let mut nonce = [0; 12];

    stream
        .lock()
        .await
        .read(&mut nonce)
        .await
        .expect("Can't get nonce!");
    let mut bytes = Vec::new();

    let mut data = [0; 16400];

    while let Ok(n) = stream.lock().await.read_exact(&mut data).await {
        if n == 0 {
            break;
        } else {
            b2f(&bytes, file.try_clone().await.unwrap())
                .await
                .expect("Not write in file!");
            bytes = match dec_data(secret, data[..n].to_vec(), nonce) {
                Ok(data) => data,
                Err(_) => {
                    panic!("Data not decrypted!");
                }
            };
        }
    }
    del_zeros(&mut bytes);

    b2f(&bytes, file.try_clone().await.unwrap())
        .await
        .expect("Not write in file!");
}
