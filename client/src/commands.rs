use std::fs::File;
use std::{process, sync::Arc};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use f2b::b2f;
use crate::{get_secret, dec_data};
mod command;
mod ls;
mod length;
use ls::get_entries;
use length::get_length;
use command::send_command;
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
    
    pub fn execute(self, stream: TcpType, flag: String){
        match self {
            Command::List => Ls.command_execution(stream, flag),
            Command::Download => Get.command_execution(stream, flag),
            Command::Upload => Put.command_execution(stream, flag), 
            Command::Exit => Exit.command_execution(stream, flag),
            Command::GetHelp => Help.command_execution(stream, flag),
        };
    }
}

trait Exec{
    fn command_execution(&self, stream: TcpType, flag: String);
}

impl Exec for Help{
    fn command_execution(&self, _stream: TcpType, _flag: String) {
        println!("
        get [FILE] - download file
        put [FILE] - upload file
        ls - print file list
        help - print this text
        exit, quit, q - stop program\n");
    }
}

impl Exec for Get{
    fn command_execution(&self, stream: TcpType, flag: String) {
        tokio::spawn(async move{
            send_command(stream.clone(), String::from("get")).await;

            stream.lock().await
                .write(flag.as_bytes())
                .await.expect("Filename not sent!");

            let mut file_len = stream.lock().await
                .read_i64()
                .await.expect("Can't get file length!"); 

            let secret = get_secret(stream.clone())
                .await.expect("Key not generated!");

            let file = File::options()
                .write(true)
                .read(true)
                .create(true)
                .open(flag)
                .expect("File not created!");

            while file_len > 0 {

                let mut data = Vec::new();

                stream.lock().await
                    .read_buf(&mut data)
                    .await.expect("Can't get data!");

                let mut counter = 0;

                while counter < data.len(){
                    let bytes = dec_data(secret, &data[counter..counter+16])
                        .expect("Data not decrypted!");
                    b2f(&bytes, &file).expect("Not write in file!");
                    counter += 16;
                }
                file_len -= 64;
            }         
        });
    }
}

impl Exec for Put{
    fn command_execution(&self, _stream: TcpType, _flag: String) {
        todo!()
    }
}

impl Exec for Ls{
    fn command_execution(&self, stream: TcpType, _flag: String) {
        tokio::spawn(async move{
            send_command(stream.clone(), String::from("ls")).await;

            let mut message_len = get_length(stream.clone()).await;

            let secret = get_secret(stream.clone())
                .await.expect("Key not sent!");

            println!("{}", String::from_utf8_lossy(
                &get_entries(stream.clone(), &mut message_len, secret).await));
        });
    }
}

impl Exec for Exit{
    fn command_execution(&self, _stream: TcpType, _flag: String) {
        process::exit(0x0100);
    }
}