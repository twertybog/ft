use tokio::{sync::Mutex, io::AsyncReadExt, net::TcpStream};
use std::sync::Arc;
use crate::dec_data;

pub async fn get_entries(stream: Arc<Mutex<TcpStream>>, 
    message_len: &mut i64, secret: [u8;32]) -> Vec<u8>
{
    let mut list: Vec<u8> = Vec::new();

    while *message_len > 0 {
        let mut message = [0;1024];
        let mut nonce = [0;12];

        stream.lock().await
            .read(&mut nonce).await
            .expect("Can't get nonce!");

        stream.lock().await
            .read(&mut message).await
            .expect("Can't read the message!");
        
        dec_data(secret, message.to_vec(), nonce)
                .expect("Data not decrypted!")
                .into_iter()
                .map(|byte| byte).collect_into(&mut list);
        *message_len -= 1024;
    }
    list
}