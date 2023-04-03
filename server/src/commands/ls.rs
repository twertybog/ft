use crate::enc_data;
use crate::gen_nonce;
use std::fs::read_dir;
use std::io::IoSlice;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::{net::TcpStream, sync::Mutex};

pub async fn send_entries(entries: &[u8], secret: [u8; 32], stream: Arc<Mutex<TcpStream>>) {
    let nonce = gen_nonce();

    let data = enc_data(secret, entries.to_vec(), nonce)
        .expect("Data not encrypted!");

    let bufs = [IoSlice::new(&nonce), IoSlice::new(&data)];

    stream.lock().await
        .write_vectored(&bufs).await
        .expect("Packet not sent!");
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
