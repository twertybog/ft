#[derive(Debug)]
pub enum Command{
    LS,
    GET,
    PUT,
    HELP
}

impl Command{
    //return Command
    pub fn command_handler(command: &str) -> Option<Command>{
        let command: String = command
        .trim()
        .parse()
        .expect("Command can't be converted into \"&str\"");

        match command.as_str(){
            "ls" => Some(Command::LS),
            "get" => Some(Command::GET),
            "put" => Some(Command::PUT),
            "help" => Some(Command::HELP),
            _ => None
        }
    }

    pub fn command_help(){
        println!("
        get [FILE] - download file
        put [FILE] - upload file
        ls - print file list
        help - print this text\n");
    }
}