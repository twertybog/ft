use std::sync::Arc;
use tokio::{io::AsyncReadExt, net::TcpStream, sync::Mutex};

pub async fn get_length(stream: Arc<Mutex<TcpStream>>) -> i64{
    let mut message_len = Vec::new();

    stream
        .lock()
        .await
        .read_buf(&mut message_len)
        .await
        .expect("Length not get!");
    let message_len = String::from_utf8_lossy(&message_len)
        .to_string()
        .trim()
        .parse::<i64>()
        .unwrap_or(0);
    message_len
}
