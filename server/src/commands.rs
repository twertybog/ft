use std::sync::Arc;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::{net::TcpStream, io::AsyncReadExt};
use tokio::sync::Mutex;
mod command;
mod length;
mod ls;
mod get;
pub use length::{send_length, get_length};
use ls::{send_entries, get_entries};
use get::send_file;
pub use command::get_command;
use crate::{get_secret};
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
    fn command_execution(&self, stream: Arc<Mutex<TcpStream>>){
        tokio::spawn(async move {
            let mut filename = Vec::new();

            stream.lock().await
                .read_buf(&mut filename)
                .await.expect("Can't get filename!");

            let filename = &format!("{}", String::from_utf8_lossy(&filename));

            let metadata = fs::metadata(filename)
                .await.expect("File not exist!");
            
            if metadata.is_file(){
                let file_len = metadata.len();
                println!("Is file!");
                stream.lock().await
                    .write_u64(file_len)
                    .await.expect("File length not sent!");

                let secret = get_secret(stream.clone()).await
                    .expect("Key not generated!");
                send_file(filename, stream.clone(), file_len, secret).await;      
            }
        });
    }
}

impl Exec for Put{
    fn command_execution(&self, _stream: Arc<Mutex<TcpStream>>){
        todo!()
    }
}

