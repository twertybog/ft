use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpStream;
use crate::{get_secret, dec_data, Command};


pub async fn get_command(stream: Arc<Mutex<TcpStream>>) -> Result<(), Box<dyn Error>>{
    let command = tokio::spawn(async move {
        let _ = stream.lock().await
            .read_i64().await
            .expect("Length not get!");

        let secret = get_secret(stream.clone()).await
            .expect("Key not recieved!");

        let mut command = [0;19];

        let mut nonce = [0;12];
            
        stream.lock().await
            .read(&mut nonce).await
            .expect("Can't get nonce!");

        stream.lock().await
            .read(&mut command).await
            .expect("Can't read the message!");

        let bytes = dec_data(secret, command.to_vec(), nonce)
            .expect("Data not decrypted!");

        let command = String::from_utf8_lossy(&bytes).to_string();

        Command::command_handler(&command, stream.clone());
    });
    Ok(command.await?)
}