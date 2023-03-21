use crate::{dec_data, get_secret};
use f2b::b2f;
use tokio::fs::OpenOptions;
use std::fs::File;
use std::{process, sync::Arc};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
mod command;
mod ls;
use command::send_command;
use ls::get_entries;
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
pub enum Command {
    List,
    Download,
    Upload,
    GetHelp,
    Exit,
}

impl Command {
    pub fn command_handler(command: &str) -> Option<Command> {
        match command {
            "ls" => Some(Command::List),
            "get" => Some(Command::Download),
            "put" => Some(Command::Upload),
            "help" => Some(Command::GetHelp),
            "q" | "exit" | "quit" => Some(Command::Exit),
            _ => None,
        }
    }

    pub fn execute(self, stream: TcpType, flag: String) {
        match self {
            Command::List => Ls.command_execution(stream, flag),
            Command::Download => Get.command_execution(stream, flag),
            Command::Upload => Put.command_execution(stream, flag),
            Command::Exit => Exit.command_execution(stream, flag),
            Command::GetHelp => Help.command_execution(stream, flag),
        };
    }
}

trait Exec {
    fn command_execution(&self, stream: TcpType, flag: String);
}

impl Exec for Help {
    fn command_execution(&self, _stream: TcpType, _flag: String) {
        println!(
            "
        get [FILE] - download file
        put [FILE] - upload file
        ls - print file list
        help - print this text
        exit, quit, q - stop program\n"
        );
    }
}

impl Exec for Get {
    fn command_execution(&self, stream: TcpType, flag: String) {
        tokio::spawn(async move {
            send_command(stream.clone(), String::from("get")).await;

            stream
                .lock()
                .await
                .write(flag.as_bytes())
                .await
                .expect("Filename not sent!");

            let mut file_len = stream
                .lock()
                .await
                .read_i64()
                .await
                .expect("Can't get file length!");

            let file = OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .open(flag)
                .await
                .expect("File not created!");

            let secret = get_secret(stream.clone())
                .await
                .expect("Key not generated!");

            let mut errors = Vec::new();

            while file_len > 0 {
                let mut nonce = [0; 12];

                let mut data = vec![0; 16400];

                stream
                    .lock()
                    .await
                    .read(&mut nonce)
                    .await
                    .expect("Can't get nonce!");

                stream
                    .lock()
                    .await
                    .read(&mut data)
                    .await
                    .expect("Can't get data!");

                let bytes = match dec_data(secret, data.to_vec(), nonce){
                    Ok(data) => data,
                    Err(_) => {
                        println!("Not decrypted: {}", file_len);
                        errors.push(file_len);
                        vec![]
                    }
                };

                b2f(&bytes, file.try_clone().await.unwrap()).await.expect("Not write in file!");

                file_len -= 16384;
            }
            println!("Amount of errors: {}", errors.len());
            println!("Success!")
        });
    }
}

impl Exec for Put {
    fn command_execution(&self, _stream: TcpType, _flag: String) {
        todo!()
    }
}

impl Exec for Ls {
    fn command_execution(&self, stream: TcpType, _flag: String) {
        tokio::spawn(async move {
            send_command(stream.clone(), String::from("ls")).await;

            let mut message_len = stream
                .lock()
                .await
                .read_i64()
                .await
                .expect("Can't get length");

            let secret = get_secret(stream.clone()).await.expect("Key not sent!");

            println!(
                "{}",
                String::from_utf8_lossy(
                    &get_entries(stream.clone(), &mut message_len, secret).await
                )
            );
        });
    }
}

impl Exec for Exit {
    fn command_execution(&self, _stream: TcpType, _flag: String) {
        process::exit(0x0100);
    }
}

async fn errors_fix(stream: TcpType, mut errors: Vec<i64>, file: File, secret: [u8;32]){
    while errors.len() > 0{
        for i in (0..errors.len()).rev(){
            stream.lock().await
                .write_i64(errors[i]).await
                .expect("Packet number not sent!");
            let mut nonce = [0;12];
            
            stream.lock().await
                .read(&mut nonce).await
                .expect("Nonce not get!");

            let mut data = [0;16400];

            stream.lock().await
                .read(&mut data).await
                .expect("Can't get data");

            let bytes = match dec_data(secret, data.to_vec(), nonce) {
                Ok(data) => {
                    errors.remove(i);
                    data
                },
                Err(_) => vec![]
            };
        }
    }
}
