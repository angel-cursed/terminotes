mod write;

use crossterm::{
    execute,
    terminal::{self,ClearType},
    cursor,
    style::Stylize,
};

use std::io::{self, Write};

use json;

fn main() {
    let mut stdout = io::stdout();

    let _ = execute!(stdout, terminal::Clear(ClearType::All));

    let _ = execute!(stdout, cursor::MoveTo(0,0));

    println!("{}", r"
     _______                  _   _   _       _
    |__   __|                (_) | \ | |     | |
       | | ___ _ __ _ __ ___  _  |  \| | ___ | |_ ___  ___
       | |/ _ \ '__| '_ ` _ \| | | . ` |/ _ \| __/ _ \/ __|
       | |  __/ |  | | | | | | | | |\  | (_) | ||  __/\__ \
       |_|\___|_|  |_| |_| |_|_| |_| \_|\___/ \__\___||___/


    ".red().bold());
    
    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command);

        let command: Vec<&str> = command.split_whitespace().collect();

        match command[0] {
            "write" => {let input =  write::write();
                println!("{}", input);},
            "exit" => break,
            _ => println!("Invalid command. Use 'write' or 'exit'."),
        }
        
    }
}
