use crate::enc_data;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::{net::TcpStream, sync::Mutex};
use std::fs::read_dir;
use crate::gen_nonce;

pub async fn send_entries(entries: &[u8], secret: [u8; 32], stream: Arc<Mutex<TcpStream>>) {
    let mut counter = 0;

    while entries.len() > counter {
        let nonce = gen_nonce();

        let data = enc_data(secret, {
            if entries.len() < counter + 1024 {
                entries[counter..].to_vec()
            } else {
                entries[counter..(counter + 1024)].to_vec()
            }
        }, nonce)
        .expect("Data not encrypted!");

        stream.lock().await
            .write(&nonce).await
            .expect("Nonce not sent!");
        
        stream
            .lock()
            .await
            .write(&data)
            .await
            .expect("Data not sent");
        counter += 1024;
    }
}

pub fn get_entries() -> String {
    let mut entries = String::new();
    for entry in read_dir(".").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        entries.push_str(&path.to_str().unwrap_or("./None")[2..]);
        entries.push(' ');
    }
    entries
}
