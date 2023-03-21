use crate::{enc_data, gen_nonce};
use f2b::f2b_part;
use std::sync::Arc;
use tokio::io::{AsyncWriteExt};
use tokio::{net::TcpStream, sync::Mutex};

pub async fn send_file(
    filename: &str,
    stream: Arc<Mutex<TcpStream>>,
    file_len: u64,
    secret: [u8; 32],
) {
    let mut counter = 0;

    while file_len > counter {
        let data = f2b_part(&filename, counter, 16384).await
            .expect("Can't get a part from file!");

        let nonce = gen_nonce();

        let data = enc_data(secret, data, nonce).expect("Data not encrypted!");

        stream
            .lock()
            .await
            .write(&nonce)
            .await
            .expect("Nonce not sent!");

        stream
            .lock()
            .await
            .write(&data)
            .await
            .expect("Data not sent!");
        counter += 16384;
    }
}
