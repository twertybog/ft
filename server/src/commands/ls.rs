use crate::enc_data;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::{net::TcpStream, sync::Mutex};
use std::fs::read_dir;

pub async fn send_entries(entries: &[u8], secret: [u8; 32], stream: Arc<Mutex<TcpStream>>) {
    let mut counter = 0;

    while entries.len() > counter {
        let data = enc_data(secret, {
            if entries.len() < counter + 16 {
                entries[counter..].to_vec()
            } else {
                entries[counter..(counter + 16)].to_vec()
            }
        })
        .expect("Data not encrypted!");

        stream
            .lock()
            .await
            .write(&data)
            .await
            .expect("Data not sent");
        counter += 16;
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
