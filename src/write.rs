use std::io::{self, Write};
use serde_json::Value;

pub fn write() -> Value {
    let mut input = String::new();

    println!("Enter your text (write END on an empty line to finish)\n");

    return write_loop(&mut input);
}

pub fn edit_note(note: String) -> Value {
    let mut input = note;

    println!("Enter your text (write END on an empty line to finish)\n");

    for line in input.lines() {
        print!("    > ");
        println!("{}", line);
        io::stdout().flush().expect("Failed to flush");
    }

    let _ = write_loop(&mut input);

    return Value::String(input);
}

pub const fn get_help_message<'a>() -> &'a str {
    return "Usage:\n
    write <note> - Edit a note\n
    see <note> - View a specific note\n
    remove <note> - Remove a specific note\n
    create <title> - Create a new note with the given title\n
    list - List all the current notes\n
    exit - Exit the program\n";
}

fn write_loop( input: &mut String) -> Value {
        loop {
        let mut line = String::new();

        print!("    > ");
        io::stdout().flush().expect("Failed to flush");

        io::stdin().read_line(&mut line).expect("Failed to read line");

        let trimmed = line.trim();

        if trimmed == "END" {
            break;
        }

        input.push_str(trimmed);
        input.push('\n');
    }

    let input = input.clone();
    return Value::String(input);
}