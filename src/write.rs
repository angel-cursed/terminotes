use std::io::{self, Write};
use serde_json::Value;


use crossterm::{
    queue,
    execute,
    terminal::{ClearType, Clear},
    cursor::MoveTo,
    event::{self,Event, KeyCode, KeyEvent},
};

pub fn write() -> Value {
    let input = String::new();

    return write_loop(input);
}

pub fn edit_note(note: String) -> Value {
    let input = note;

    return write_loop(input)
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

fn write_loop(input: String) -> Value {

    // let _ = terminal::enable_raw_mode();
    let mut stdout = io::stdout();

    let mut lines : Vec<String> = Vec::new();
    if !input.is_empty() {
        for line in input.lines() {
            lines.push(line.to_string() + "\n");
        }
    }

    let mut line = String::new();
    if !lines.is_empty() {
        line = lines[lines.len() - 1].clone();
    }
    let mut line_index = lines.len();
    if line_index > 0 {
        line_index -= 1;
    }

    let mut index: usize = 0;

    if !lines.is_empty(){
        index = lines[line_index].len();
    }
        loop {
            execute!(
                stdout,
                MoveTo(0,0),
                Clear(ClearType::All)
            ).expect("Failed to clear terminal");

            println!("Enter your text (Use ESC to finish) {:?}\n", lines);

            for line in lines.iter() {
                let _ = write!(stdout,"    > {line}");
            }
            let _ = queue!(stdout, MoveTo((index + 6) as u16 , (line_index + 2) as u16));
            stdout.flush().expect("Failed to flush");
            match event::read() {
                Ok(Event::Key(KeyEvent { code, .. })) => {
                    match code {
                        KeyCode::Esc => {
                            break;
                        },
                        KeyCode::Backspace => {
                            if index > 0 {
                                line.remove(index - 1);
                                index -= 1;
                            } else if line_index > 0 {
                                line.clear();
                                lines.remove(line_index);
                                line_index -= 1;
                                lines[line_index].pop();
                                line = lines[line_index].clone();
                                index = line.len();
                            }
                        },
                        KeyCode::Char(c) => {
                            line.insert(index, c);
                            index += 1;
                        },
                        KeyCode::Enter => {
                            if lines.is_empty(){
                                lines.push("\n".to_string());
                            }
                            line_index += 1;
                            line.clear();

                            if line_index >= lines.len() {
                                lines.push(line.clone());
                            } else {
                                lines.insert(line_index, line.clone());
                                lines[line_index].pop();
                            }
                            index = 0;
                        },
                        KeyCode::Up => {
                            if line_index > 0 {
                                line_index -= 1;
                                lines[line_index].pop();
                                line = lines[line_index].clone();
                                index = line.len();
                            }
                        },
                        KeyCode::Down => {
                            line_index += 1;

                            if line_index < lines.len(){
                                lines[line_index].pop();
                                line = lines[line_index].clone();
                                index = line.len();
                            }else {
                                line_index -= 1;
                            }
                        },
                        KeyCode::Left => {
                            if index > 0 {
                                index -= 1;
                            } else if line_index > 0 {
                                line_index -= 1;
                                lines[line_index].pop();
                                line = lines[line_index].clone();
                                index = line.len();
                            }
                        },
                        KeyCode::Right => {
                            if index < line.len() {
                                index += 1;
                            } else if line_index + 1 < lines.len() {
                                line_index += 1;
                                lines[line_index].pop();
                                line = lines[line_index].clone();
                                index = line.len();

                            }
                        },
                        _ => {}
                    }
                },
                _ => {
                    eprintln!("Error");
                    break;
                }
            }
        if line_index >= lines.len() {
            lines.resize(line_index + 1, "\n".to_string());
        }
        lines[line_index] = line.clone();
        lines[line_index].push('\n');

    }

    // let _ = terminal::disable_raw_mode();

    let mut collected = String::new();

    for line in lines.iter() {
        collected.push_str(&line)
    }

    collected.pop();

    return Value::String(collected);
}