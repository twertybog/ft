use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpStream;
use crate::{get_secret, dec_data, Command};


pub async fn get_command(stream: Arc<Mutex<TcpStream>>) -> Result<(), Box<dyn Error>>{
    let command = tokio::spawn(async move {
        let mut message_len: Vec<u8> = Vec::new();

        stream.lock().await
            .read_buf(&mut message_len).await
            .expect("Can't read the message length!");

        let secret = get_secret(stream.clone()).await
            .expect("Key not recieved!");

        let mut message_len = String::from_utf8_lossy(&message_len)
            .trim().parse::<i64>().unwrap_or(0);

        let mut command: Vec<u8> = Vec::new();

        while message_len > 0 {
            let mut message = Vec::new();

            stream.lock().await
                .read_buf(&mut message).await
                .expect("Can't read the message!");
            dec_data(secret, message).expect("Data not decrypted!")
                .into_iter()
                .map(|byte| byte).collect_into(&mut command);
            message_len -= 16;
        }
        let command = String::from_utf8_lossy(&command).to_string();

        let mut command_split = command.split_whitespace();

        Command::command_handler(command_split.next().unwrap_or(" "), stream.clone());
    });

    Ok(command.await?)
}