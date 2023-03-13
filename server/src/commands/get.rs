use tokio::{sync::Mutex, net::TcpStream};
use tokio::io::AsyncWriteExt;
use f2b::f2b_part;
use std::{sync::Arc};
use crate::enc_data;

pub async fn send_file(filename: &str, stream: Arc<Mutex<TcpStream>>, file_len: u64, secret: [u8;32]) {
    let mut counter = 0;

    while file_len > counter {
        let data = f2b_part(&filename, counter, 16)
            .expect("Can't get a part from file!");
        
        let data = enc_data(secret, data).expect("Data not encrypted!");
        

        stream
            .lock()
            .await
            .write(&data)
            .await
            .expect("Data not sent!");

        counter += 16;
    }
}