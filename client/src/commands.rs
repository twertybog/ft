use crate::{GET, LS, PUT};
use api::{get_file, send_secret, send_file};
use std::{process, sync::Arc};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
mod command;
mod ls;
use command::send_command;
use ls::get_entries;

type TcpType = Arc<Mutex<TcpStream>>;

struct Ls;

struct Get;

struct Put;

struct Help;

struct Exit;

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
            send_command(stream.clone(), GET).await;
            stream
                .lock()
                .await
                .write(flag.as_bytes())
                .await
                .expect("Filename not sent!");

            get_file(stream.clone(), &flag).await;

            println!("Success!")
        });
    }
}

impl Exec for Put {
    fn command_execution(&self, stream: TcpType, flag: String) {
        tokio::spawn(async move{
            send_command(stream.clone(), PUT).await;

            stream.lock().await
                .write(&flag.as_bytes()).await
                .expect("FIlename not sent");

            send_file(&flag, stream.clone()).await;

            println!("Success!");
        });
    }
}

impl Exec for Ls {
    fn command_execution(&self, stream: TcpType, _flag: String) {
        tokio::spawn(async move {
            send_command(stream.clone(), LS).await;

            let secret = send_secret(stream.clone()).await.expect("Key not sent!");

            println!(
                "{}",
                String::from_utf8_lossy(&get_entries(stream.clone(), secret).await)
            );
        });
    }
}

impl Exec for Exit {
    fn command_execution(&self, _stream: TcpType, _flag: String) {
        process::exit(0x0100);
    }
}
