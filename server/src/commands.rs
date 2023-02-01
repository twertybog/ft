use tokio::{fs::read_dir};
use tokio::io::{ReadHalf, WriteHalf};
use tokio::TcpStream;
struct Ls;

struct Get;

struct Put;

pub enum Command{
    List,
    Download,
    Upload,
}

trait Exec{
    fn command_execution(&self);
}

impl Command{
    pub fn command_handler(command: &str) -> Option<Command>{
        match command{
            "ls" => Some(Command::List),
            "get" => Some(Command::Download),
            "put" => Some(Command::Upload),
            _ => None
        }
    }
    
    pub fn execute<T>(self, _flags: T){
        match self {
            Command::List => Ls.command_execution(),
            Command::Download => Get.command_execution(),
            Command::Upload => Put.command_execution(), 
        };
    }
}

impl Exec for Ls{
    fn command_execution(&self, rd: ReadHalf<TcpStream>, wr: WriteHalf<TcpStream>){
        let mut entries = read_dir(".").await?;
        let mut directories = String::new();

        while let Some(entry) = entries.next_entry().await? {
            directories.push_str(&entry.path().to_string_lossy()[2..]);
            directories.push(' ');
        }
        println!("{}", directories);
        
        todo!()
    }
}

impl Exec for Get{
    fn command_execution(&self){
        todo!()
    }
}

impl Exec for Put{
    fn command_execution(){
        todo!()
    }
}