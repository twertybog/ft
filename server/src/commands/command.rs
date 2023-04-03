use api::{del_zeros, get_secret};
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpStream;
use crate::{dec_data, Command};


pub async fn get_command(stream: Arc<Mutex<TcpStream>>,) -> Result<(), Box<dyn Error>>{
    let command = tokio::spawn(async move {
        let secret = get_secret(stream.clone()).await
            .expect("Key not recieved!");

        let mut command = Vec::new();

        let mut nonce = [0;12];
            
        stream.lock().await
            .read(&mut nonce).await
            .expect("Can't get nonce!");

        stream.lock().await
            .read_buf(&mut command).await
            .expect("Can't read the message!");

        let mut bytes = dec_data(secret, command.to_vec(), nonce)
            .expect("Data not decrypted!");
        
        del_zeros(&mut bytes);

        let command = String::from_utf8_lossy(&bytes).to_string();

        Command::command_handler(&command, stream.clone());
    });
    Ok(command.await?)
}