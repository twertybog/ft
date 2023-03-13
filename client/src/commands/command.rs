use crate::{enc_data, get_secret};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::{net::TcpStream, sync::Mutex};

pub async fn send_command(stream: Arc<Mutex<TcpStream>>, command: String) {
    stream
        .lock()
        .await
        .write(format!("16").as_bytes())
        .await
        .expect("Length not sent!");

    let secret = get_secret(stream.clone()).await.expect("Key not sent!");

    let message = enc_data(secret, command.as_bytes().to_vec()).expect("Data not encrypted!");

    stream
        .lock()
        .await
        .write(&message)
        .await
        .expect("Data not sent!");
}
