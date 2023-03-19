use crate::{enc_data, gen_nonce};
use f2b::f2b_part;
use std::borrow::Cow;
use std::sync::Arc;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::{net::TcpStream, sync::Mutex};

pub async fn send_file(
    filename: &str,
    stream: Arc<Mutex<TcpStream>>,
    file_len: u64,
    secret: [u8; 32],
) {
    let mut counter = 0;

    while file_len > counter {
        loop {
            let data = f2b_part(&filename, counter, 1024)
                .expect("Can't get a part from file!");

            let nonce = gen_nonce();

            let data = enc_data(secret, data, nonce)
                .expect("Data not encrypted!");

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

            let mut success = Vec::new();
            
            stream.lock().await
                    .read_buf(&mut success).await
                    .expect("Succes message not read");
                match String::from_utf8_lossy(&success)
            {
                Cow::Borrowed("true") => {
                    break;
                },
                _ => {
                    println!("Not sent!");
                    continue;
                }                
            };
        }
        counter += 1024;
    }
}
