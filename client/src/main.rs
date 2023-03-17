#![feature(iter_collect_into)]
use tokio::sync::Mutex;
use tokio::net::TcpStream;
use std::sync::Arc;
use std::io as sio;
mod commands;
use commands::Command;
mod secret;
pub use secret::get_secret;
pub use api::{gen_nonce, enc_data, dec_data};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop{
        let mut command = String::new();

        sio::stdin().read_line(&mut command)?;

        let mut command = command.split_whitespace();

        match Command::command_handler(&command.next().unwrap_or(" ")) {
            Some(com) => {
                let socket = TcpStream::connect("127.0.0.1:4956").await?;

                let stream = Arc::new(Mutex::new(socket));
                
                com.execute(stream.clone(), command.next().unwrap_or(" ").to_string());
            },
            None => ()
        };
    }
}