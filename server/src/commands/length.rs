use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::{net::TcpStream, sync::Mutex};

pub async fn send_length(stream: Arc<Mutex<TcpStream>>, length: usize) {
    stream
        .lock()
        .await
        .write(format!("{}", length).as_bytes())
        .await
        .expect("Length not sent!");
}

pub async fn get_length(stream: Arc<Mutex<TcpStream>>) -> i64 {
    let mut message_len: Vec<u8> = Vec::new();

    stream
        .lock()
        .await
        .read_buf(&mut message_len)
        .await
        .expect("Can't read the message length!");

    String::from_utf8_lossy(&message_len)
        .trim()
        .parse::<i64>()
        .unwrap_or(0)

}
