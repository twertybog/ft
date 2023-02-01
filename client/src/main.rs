use rand_core::OsRng;
use k256::{EncodedPoint, PublicKey, ecdh::EphemeralSecret};
use std::io as sio;
use tokio::sync::Mutex;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt, WriteHalf};
use std::sync::Arc;
mod commands;
mod secret;
use commands::Command;
use secret::get_secret;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = TcpStream::connect("127.0.0.1:4956").await?;

    let stream = Arc::new(Mutex::new(socket));

    println!("Secret: {:?}", get_secret(stream.clone()).await);

    loop{
        let mut command = String::new();

        sio::stdin().read_line(&mut command)?;

        let mut command = command.split_whitespace();

        match Command::command_handler(&command.nth(0).unwrap()) {
            Some(com) => com.execute(command),
            None => println!("Command not found!")
        };
    }
}