use std::io::{self, Write};

pub fn write() -> String {
    let mut input = String::new();

    println!("Enter your text (write END on an empty line to finish)\n");

    loop {
        let mut line = String::new();

        print!("> ");
        io::stdout().flush().expect("Failed to flush");

        io::stdin().read_line(&mut line).expect("Failed to read line");

        let trimmed = line.trim();
        if trimmed == "END" {
            break;
        }

        input.push_str(trimmed);
        input.push('\n');
    }

    return input;
}