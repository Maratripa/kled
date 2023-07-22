use std::{
    env,
    io::{stdin, BufRead},
    process,
};

mod functions;
mod parser;

use functions::{append_line, delete_line, insert_line, replace_line};
use parser::parse_command;

fn main() {
    if env::args().len() < 2 {
        eprintln!("No file provided");
        return;
    }

    // we know that is at least length 2, so its safe to unwrap
    let path = env::args().nth(1).unwrap();

    for line in stdin().lock().lines() {
        let line = line.expect("failed to read stdin line");
        let tokens = parse_command(&line);

        match tokens[0].as_str() {
            "exit" => {
                process::exit(0);
            }
            "replace" => {
                if tokens.len() < 3 {
                    eprintln!("Not enough arguments, replace command takes a line number and the content to replace the line");
                    eprintln!("Usage: replace [line number] [new line content]");
                    process::exit(2);
                }

                let line_number: usize = tokens[1]
                    .parse()
                    .expect("[line number] argument not a number");
                let new_content = &tokens[2];

                replace_line(&path, line_number, new_content);
            }
            "delete" => {
                if tokens.len() < 2 {
                    eprintln!("Not enough arguments, delete command takes a line number to delete");
                    eprintln!("Usage: delete [line number]");
                    process::exit(2);
                }

                let line_number: usize = tokens[1]
                    .parse()
                    .expect("[line number] argument not a number");

                delete_line(&path, line_number);
            }
            "append" => {
                if tokens.len() < 2 {
                    eprintln!("Not enough arguments, append command takes the content to write to append to the end.");
                    eprintln!("Usage: append [new content]");
                    process::exit(2);
                }

                let new_content = &tokens[1];

                append_line(&path, new_content);
            }
            "insert" => {
                if tokens.len() < 3 {
                    eprintln!("Not enough arguments, insert command takes a line number and the content to insert at that position.");
                    eprintln!("Usage: insert [line number] [new content]");
                    process::exit(2);
                }

                let line_number: usize = tokens[1]
                    .parse()
                    .expect("[line number] argument not a number");

                let new_content = &tokens[2];

                insert_line(&path, line_number, new_content);
            }
            _ => {
                eprintln!("Command '{}' not found", &tokens[0]);
                process::exit(1);
            }
        }
    }
}
