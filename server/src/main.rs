#![feature(iter_collect_into)]
use std::{sync::Arc, error::Error};
use tokio::{
    sync::Mutex,
    net::TcpListener,
};
mod secret;
mod commands;
pub use secret::get_secret;
pub use api::{gen_nonce, enc_data, dec_data};
pub use commands::{get_command, Command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:4956").await?;

    loop {
        let (socket, _address) = listener.accept().await?;

        let stream = Arc::new(Mutex::new(socket));
        
        match get_command(stream.clone()).await {
            Ok(_) => (),
            Err(_) => continue
        };
    }
}