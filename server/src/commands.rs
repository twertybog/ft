use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::{io::AsyncReadExt, net::TcpStream};
mod command;
mod ls;
use api::{get_secret, send_file, get_file};
pub use command::get_command;
use ls::{get_entries, send_entries};
struct Ls;

struct Get;

struct Put;

pub struct Command;

trait Exec {
    fn command_execution(&self, stream: Arc<Mutex<TcpStream>>);
}

impl Command {
    pub fn command_handler(command: &str, stream: Arc<Mutex<TcpStream>>) {
        match command {
            "ls" => Ls.command_execution(stream),
            "get" => Get.command_execution(stream),
            "put" => Put.command_execution(stream),
            _ => (),
        }
    }
}

impl Exec for Ls {
    fn command_execution(&self, stream: Arc<Mutex<TcpStream>>) {
        let entries = get_entries();
        tokio::spawn(async move {
            let entries = entries.as_bytes();

            let secret = get_secret(stream.clone())
                .await
                .expect("Key not generated!");

            send_entries(entries, secret, stream).await;
        });
    }
}

impl Exec for Get {
    fn command_execution(&self, stream: Arc<Mutex<TcpStream>>) {
        tokio::spawn(async move {
            let mut filename = Vec::new();

            stream
                .lock()
                .await
                .read_buf(&mut filename)
                .await
                .expect("Can't get filename!");

            let filename = &format!("{}", String::from_utf8_lossy(&filename));

            send_file(filename, stream.clone()).await;
        });
    }
}

impl Exec for Put {
    fn command_execution(&self, stream: Arc<Mutex<TcpStream>>) {
        tokio::spawn(async move{
            let mut bytes = Vec::new();

            stream.lock().await
                .read_buf(&mut bytes)
                .await
                .expect("Can't get filename!");

            let filename = String::from_utf8_lossy(&bytes);

            get_file(stream.clone(), &filename).await;
        });
    }
}
