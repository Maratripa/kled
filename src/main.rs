use std::{
    env,
    fs::{self, File},
    io::{stdin, BufRead, BufReader, BufWriter, Write},
    process,
};

mod parser;

use parser::parse_command;

fn replace_line(path: &str, line_number: usize, new_content: &str) {
    // open file
    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);

    // create temp file to write to
    // TODO: make unique name for temp file
    let new_file_path = "temptemp";
    let new_file = File::create(new_file_path).expect("failed to create new file");
    let mut writer = BufWriter::new(new_file);

    // iterate over the lines and modify the desired line
    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("failed to read line");

        if i + 1 == line_number {
            // modify desired line
            writer
                .write_all(new_content.as_bytes())
                .expect("failed to write line");
        } else {
            // copy the original line
            writer
                .write_all(line.as_bytes())
                .expect("failed to write line");
        }

        // add newline character after each line
        writer
            .write_all("\n".as_bytes())
            .expect("failed to write character");
    }

    // flush the writer to ensure all data is written to the file
    writer.flush().expect("failed to flush");

    // replace the original file with the new file
    fs::rename(new_file_path, path).expect("failed to replace file");
}

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
