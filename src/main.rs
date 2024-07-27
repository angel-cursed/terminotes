mod write;

use std::collections::HashMap;
use crossterm::{
    execute,
    terminal::{self,ClearType},
    cursor,
    style::Stylize,
};

use std::io::{self, Write, BufReader};

use std::fs::File;

use serde_json::{Result, Value};

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

    const HELP_MESSAGE: &str = write::get_help_message();

    let mut file = File::open("data/notes.json").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut notes = HashMap::new();

    if std::fs::metadata("data/notes.json").unwrap().len() != 0 {
        let values: Value = serde_json::from_reader(reader).expect("Failed to read JSON");

        notes = match values {
            Value::Object(map) => map.into_iter().collect(),
            _ => panic!("Expected a JSON object"),
        };
    }

    see_notes(notes.clone());
    
    loop {
        print!("-> ");
        io::stdout().flush().expect("Failed to flush");
        let mut command = String::new();
        let _ = io::stdin().read_line(&mut command);

        let command: Vec<&str> = command.split_whitespace().collect();

                if command.is_empty() {
            continue;
        }

        match command[0] {
            "write" => {
                if command.len() >= 2 {
                    if notes.contains_key(command[1]) {
                        notes.insert(command[1].to_string(), write::edit_note(notes.get(command[1]).unwrap().to_string()));
                        update_json(notes.clone());
                    }else{
                        println!("Note not found\n")
                    }
                }else {
                    println!("Please provide a note for editing\n")
                }
            },

            "see" => {
                if command.len() >= 2 {
                    if notes.contains_key(command[1]){
                        see_text(command[1], notes.get(command[1]));
                    }else{
                        println!("Note not found.\n");
                    }
                }else{
                    println!("Please provide a note for reading.\n");
                }
            },

            "remove" => {
                if command.len() >= 2 {
                    if notes.contains_key(command[1]){
                        notes.remove(command[1]);
                        update_json(notes.clone());
                        println!("Note: {}, successfully removed.\n", command[1]);
                    }else{
                        println!("Note not found.\n");
                    }
                }else{
                    println!("Please provide a note for deleting.\n");
                }
            },

            "create" => {
                if command.len() >= 2 {
                    if !notes.contains_key(command[1]){
                        notes.insert(command[1].to_string(), write::write());
                        update_json(notes.clone());
                        println!("Note: {}, successfully created.\n", command[1]);
                    }else{
                        println!("Note already exists.\n");
                    }
                }else{
                    println!("Please provide a title for creating a new note.\n");
                }
            }

            "list" => see_notes(notes.clone()),

            "exit" => break,

            "help" => println!("{}\n", HELP_MESSAGE),

            _ => println!("Invalid command. Try help.\n"),
        }
        
    }
}

fn see_notes(notes: HashMap<String, Value>) {

    if notes.is_empty() {
        println!("You don't have any notes")
    }else{

        println!("These are your current notes:");

        for title in notes {
            println!("  {}", title.0)
        }
    }
}

fn see_text(title: &str, note: Option<&Value>) {
    match note {
        Some(val) => {
            if let Some(str) = val.as_str() {
                    println!("\nTitle: {}\n", title);
                    println!("Content:\n{}", str);
            }
        }
        None => println!("Note not found."),
    }
}

fn update_json(notes: HashMap<String, Value>){
    let note_str = serde_json::to_string_pretty(&notes).unwrap();

    let bytes : &[u8] = &note_str.as_bytes();

    let mut file = File::create("data/notes.json").unwrap();

    file.write_all(bytes);
}