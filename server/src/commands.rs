use std::{fs::read_dir, sync::Arc};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
mod command;
pub use command::get_command;

use crate::{get_secret, enc_data};
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
        let mut entries = String::new();
        for entry in read_dir(".").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            
            entries.push_str(&path.to_str().unwrap_or("./None")[2..]);
            entries.push(' ');
        }
        
        tokio::spawn(async move{
            let entries = entries.as_bytes();

            stream.lock().await
                .write(format!("{}", entries.len()).as_bytes())
                .await.expect("Length not sent!");

            let secret = get_secret(stream.clone())
                .await.expect("Key not generated!");

            let mut counter = 0;

            while entries.len() > counter{
                let data = 
                    enc_data(secret, {
                        if entries.len() < counter + 16{
                            entries[counter..].to_vec()
                        }
                        else {
                            entries[counter..(counter+16)].to_vec()
                        }
                    }).expect("Data not encrypted!");
                
                stream.lock().await
                    .write(&data).await
                    .expect("Data not sent");
                counter += 16;
            }
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

