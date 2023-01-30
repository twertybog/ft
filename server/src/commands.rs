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
    fn command_execution(&self){
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