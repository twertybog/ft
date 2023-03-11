#![allow(unused_imports)]
#![feature(iter_collect_into)]
use tokio::sync::Mutex;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt, WriteHalf};
use std::sync::Arc;
use std::io as sio;
mod commands;
use commands::Command;
mod cypher;
pub use cypher::{get_secret, enc_data, dec_data};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop{
        let mut command = String::new();

        sio::stdin().read_line(&mut command)?;

        let mut command = command.split_whitespace();

        match Command::command_handler(&command.nth(0).unwrap_or(" ")) {
            Some(com) => {
                let socket = TcpStream::connect("127.0.0.1:4956").await?;

                let stream = Arc::new(Mutex::new(socket));
                
                com.execute(stream.clone());
            },
            None => ()
        };
    }
}