use crate::{enc_data, gen_nonce};
use std::sync::Arc;
use api::send_secret;
use tokio::io::AsyncWriteExt;
use tokio::{net::TcpStream, sync::Mutex};

pub async fn send_command(stream: Arc<Mutex<TcpStream>>, command: [u8; 48]) {
    let nonce = gen_nonce();

    let secret = send_secret(stream.clone())
        .await
        .expect("Key not sent!");

    let message = enc_data(secret, command.to_vec(), nonce)
        .expect("Data not encrypted!");

    stream.lock().await
        .write(&nonce).await
        .expect("Nonce not sent!");

    stream
        .lock()
        .await
        .write(&message)
        .await
        .expect("Data not sent!");
}
