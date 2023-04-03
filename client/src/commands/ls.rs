use crate::dec_data;
use std::sync::Arc;
use tokio::{io::AsyncReadExt, net::TcpStream, sync::Mutex};

pub async fn get_entries(stream: Arc<Mutex<TcpStream>>, secret: [u8; 32]) -> Vec<u8> {
    let mut list: Vec<u8> = Vec::new();

    let mut message = Vec::new();
    let mut nonce = [0; 12];

    stream
        .lock()
        .await
        .read(&mut nonce)
        .await
        .expect("Can't get nonce!");

    stream
        .lock()
        .await
        .read_to_end(&mut message)
        .await
        .expect("Can't read the message!");

    dec_data(secret, message.to_vec(), nonce)
        .expect("Data not decrypted!")
        .into_iter()
        .map(|byte| byte)
        .collect_into(&mut list);
    list
}
