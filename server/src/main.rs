#![feature(iter_collect_into)]
use std::{sync::Arc, error::Error};
use tokio::{
    sync::Mutex,
    net::TcpListener,
};
mod commands;
pub use api::{gen_nonce, enc_data, dec_data};
pub use commands::{get_command, Command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args()
        .nth(1).unwrap_or(String::from("127.0.0.1:4956"));

    println!("Connected to {}", args);

    let listener = TcpListener::bind(&args).await?;
    
    loop {
        let (socket, _address) = listener.accept().await?;

        let stream = Arc::new(Mutex::new(socket));
        
        match get_command(stream.clone()).await {
            Ok(_) => (),
            Err(_) => continue
        };
    }
}