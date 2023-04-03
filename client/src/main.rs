#![feature(iter_collect_into)]
use tokio::sync::Mutex;
use tokio::net::TcpStream;
use std::sync::Arc;
use std::io as sio;
mod commands;
use commands::Command;
pub use api::{gen_nonce, enc_data, dec_data};

pub const GET: [u8;48] = [
    103, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 
    ];

pub const LS: [u8;48] = [
    108, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ];
pub const PUT: [u8;48] = [
    112, 117, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args()
        .nth(1)
        .unwrap_or(String::from("127.0.0.1:4956"));

    loop{
        let mut command = String::new();

        sio::stdin().read_line(&mut command)?;

        let mut command = command.split_whitespace();

        match Command::command_handler(&command.next().unwrap_or(" ")) {
            Some(com) => {
                let socket = TcpStream::connect(&args).await?;

                socket.set_nodelay(true).expect("No delay not set!");

                let stream = Arc::new(Mutex::new(socket));

                
                com.execute(stream.clone(), command.next().unwrap_or(" ").to_string());
            },
            None => ()
        };
    }
}