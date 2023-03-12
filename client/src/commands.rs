use std::{process, sync::Arc};
use tokio::io::{AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::{get_secret, enc_data};
mod ls;
mod length;
use ls::get_entries;
use length::get_length;
type TcpType = Arc<Mutex<TcpStream>>;

#[derive(Debug)]
struct Ls;

#[derive(Debug)]
struct Get;

#[derive(Debug)]
struct Put;

#[derive(Debug)]
struct Help;

struct Exit;

#[derive(Debug)]
pub enum Command{
    List,
    Download,
    Upload,
    GetHelp,
    Exit
}

impl Command{
    pub fn command_handler(command: &str) -> Option<Command>{
        match command{
            "ls" => Some(Command::List),
            "get" => Some(Command::Download),
            "put" => Some(Command::Upload),
            "help" => Some(Command::GetHelp),
            "q" | 
            "exit" | 
            "quit" => Some(Command::Exit),
            _ => None
        }
    }
    
    pub fn execute(self, stream: TcpType){
        match self {
            Command::GetHelp => Help.command_execution(stream),
            Command::List => Ls.command_execution(stream),
            Command::Download => Get.command_execution(stream),
            Command::Upload => Put.command_execution(stream), 
            Command::Exit => Exit.command_execution(stream),
        };
    }
}

trait Exec{
    fn command_execution(&self, stream: TcpType);
}

impl Exec for Help{
    fn command_execution(&self, _stream: TcpType) {
        println!("
        get [FILE] - download file
        put [FILE] - upload file
        ls - print file list
        help - print this text
        exit, quit, q - stop program\n");
    }
}

impl Exec for Get{
    fn command_execution(&self, _stream: TcpType) {
        todo!()
    }
}

impl Exec for Put{
    fn command_execution(&self, _stream: TcpType) {
        todo!()
    }
}

impl Exec for Ls{
    fn command_execution(&self, stream: TcpType) {
        tokio::spawn(async move{
            stream.lock().await
                .write(format!("16").as_bytes()).await
                .expect("Length not sent!");

            let mut secret = get_secret(stream.clone())
                .await.expect("Key not sent!");

            let message = enc_data(secret, "ls"
                .as_bytes().to_vec()).expect("Data not encrypted!");

            stream.lock().await
                .write(&message).await
                .expect("Data not sent!");
            let mut message_len = get_length(stream.clone()).await;

            secret = get_secret(stream.clone())
                .await.expect("Key not sent!");

            println!("{}", String::from_utf8_lossy(
                &get_entries(stream.clone(), &mut message_len, secret).await));
        });
    }
}

impl Exec for Exit{
    fn command_execution(&self, _stream: TcpType) {
        process::exit(0x0100);
    }
}