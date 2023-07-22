use std::{
    env,
    io::{stdin, BufRead},
    process,
};

mod functions;
mod parser;

use functions::replace_line;
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
            _ => {
                eprintln!("Command '{}' not found", &tokens[0]);
                process::exit(1);
            }
        }
    }
}
