use crate::{enc_data, get_secret, gen_nonce};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::{net::TcpStream, sync::Mutex};

pub async fn send_command(stream: Arc<Mutex<TcpStream>>, command: String) {
    stream
        .lock()
        .await
        .write_i64(1024)
        .await
        .expect("Length not sent!");

    let nonce = gen_nonce();

    let secret = get_secret(stream.clone()).await.expect("Key not sent!");

    let message = enc_data(secret, command.as_bytes().to_vec(), nonce)
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
