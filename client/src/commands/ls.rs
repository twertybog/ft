use tokio::{sync::Mutex, io::AsyncReadExt, net::TcpStream};
use std::sync::Arc;
use crate::dec_data;

pub async fn get_entries(stream: Arc<Mutex<TcpStream>>, 
    message_len: &mut i64, secret: [u8;32]) -> Vec<u8>
{
    let mut list: Vec<u8> = Vec::new();

    while *message_len > 0 {
        let mut message = Vec::new();

        stream.lock().await
            .read_buf(&mut message).await
            .expect("Can't read the message!");
        let mut counter = 0;
        while counter < message.len(){
            dec_data(secret, &message[counter..counter+16])
                .expect("Data not decrypted!")
                .into_iter()
                .map(|byte| byte).collect_into(&mut list);
            counter += 16;
        }
        *message_len -= 64;
    }
    list
}