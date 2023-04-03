use crate::{enc_data, gen_nonce, get_secret};
use f2b::f2b_part;
use std::sync::Arc;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::{net::TcpStream, sync::Mutex};

const SIZE: u64 = 16384;

pub async fn send_file(filename: &str, stream: Arc<Mutex<TcpStream>>) {
    let metadata = fs::metadata(filename).await.expect("File not exist!");

    if metadata.is_file() {
        let file_len = metadata.len();

        stream
            .lock()
            .await
            .write_u64(file_len)
            .await
            .expect("File length not sent!");

        let secret = get_secret(stream.clone())
            .await
            .expect("Key not generated!");

        let mut counter = 0;

        let nonce = gen_nonce();

        stream
            .lock()
            .await
            .write_all(&nonce)
            .await
            .expect("Nonce not sent!");

        while file_len > counter + SIZE {
            let data = f2b_part(&filename, counter, SIZE)
                .await
                .expect("Can't get a part from file!");

            send_packet(secret, data, stream.clone(), nonce).await;

            counter += SIZE;
        }

        let mut data = f2b_part(&filename, counter, SIZE)
            .await
            .expect("Can't get a part from file!");

        while data.len() < SIZE as usize {
            data.push(0);
        }

        send_packet(secret, data, stream.clone(), nonce).await;
    }
}

async fn send_packet(
    secret: [u8; 32],
    data: Vec<u8>,
    stream: Arc<Mutex<TcpStream>>,
    nonce: [u8; 12],
) {
    let data = enc_data(secret, data, nonce).expect("Data not encrypted!");
    stream
        .lock()
        .await
        .write_all(&data)
        .await
        .expect("Data not sent!");

    stream
        .lock()
        .await
        .flush()
        .await
        .expect("Flush not checked!");
}
