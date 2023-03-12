use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
mod command;
mod length;
mod ls;
pub use length::{send_length, get_length};
use ls::{send_entries, get_entries};
pub use command::get_command;
use crate::get_secret;
struct Ls;

struct Get;

struct Put;

pub struct Command;

trait Exec{
    fn command_execution(&self, stream: Arc<Mutex<TcpStream>>);
}

impl Command{
    pub fn command_handler(command: &str, stream: Arc<Mutex<TcpStream>>){
        match command{
            "ls" => Ls.command_execution(stream),
            "get" => Get.command_execution(stream),
            "put" => Put.command_execution(stream),
            _ => ()
        }
    }
}

impl Exec for Ls{
    fn command_execution(&self, stream: Arc<Mutex<TcpStream>>){
        let entries = get_entries();
        tokio::spawn(async move{
            let entries = entries.as_bytes();

            send_length(stream.clone(), entries.len()).await;

            let secret = get_secret(stream.clone())
                .await.expect("Key not generated!");

            send_entries(entries, secret, stream).await;
        });
    }
}

impl Exec for Get{
    fn command_execution(&self, _stream: Arc<Mutex<TcpStream>>){
        todo!()
    }
}

impl Exec for Put{
    fn command_execution(&self, _stream: Arc<Mutex<TcpStream>>){
        todo!()
    }
}

