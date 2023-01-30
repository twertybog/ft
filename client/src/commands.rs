use std::process;

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
    
    pub fn execute<T>(self, _flags: T){
        match self {
            Command::GetHelp => Help.command_execution(),
            Command::List => Ls.command_execution(),
            Command::Download => Get.command_execution(),
            Command::Upload => Put.command_execution(), 
            Command::Exit => Exit.command_execution(),
        };
    }
}

trait Exec{
    fn command_execution(&self);
}

impl Exec for Help{
    fn command_execution(&self) {
        println!("
        get [FILE] - download file
        put [FILE] - upload file
        ls - print file list
        help - print this text
        exit, quit, q - stop program\n");
    }
}

impl Exec for Get{
    fn command_execution(&self) {
        todo!()
    }
}

impl Exec for Put{
    fn command_execution(&self) {
        todo!()
    }
}

impl Exec for Ls{
    fn command_execution(&self) {
        todo!()
    }
}

impl Exec for Exit{
    fn command_execution(&self) {
        process::exit(0x0100);
    }
}