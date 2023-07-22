use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
};

use uuid::Uuid;

fn generate_unique_tempname() -> String {
    // Generate a unique identifier (UUID)
    let unique_id = Uuid::new_v4();

    // Create file name
    format!(".temp_file_{}", unique_id)
}

pub fn replace_line(path: &str, line_number: usize, new_content: &str) {
    // open file
    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);

    // create temp file to write to
    let new_file_path = generate_unique_tempname();
    let new_file = File::create(&new_file_path).expect("failed to create new file");
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

pub fn delete_line(path: &str, line_number: usize) {
    // open file
    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);

    // create temp file
    let new_file_name = generate_unique_tempname();
    let new_file = File::create(&new_file_name).expect("failed to create new file");
    let mut writer = BufWriter::new(new_file);

    for (i, line) in reader.lines().enumerate() {
        if i + 1 != line_number {
            let mut line = line.expect("failed to read line");
            line.push('\n');
            writer
                .write_all(line.as_bytes())
                .expect("failed to write line");
        }
    }
    // flush the writer to ensure all data is writtern to the file
    writer.flush().expect("failed to flush");

    // replace the original file with the new file
    fs::rename(new_file_name, path).expect("failed to replace file");
}

pub fn append_line(path: &str, new_content: &str) {
    // open file
    let mut file = OpenOptions::new()
        .append(true)
        .open(path)
        .expect("failed to open file");

    file.write_all(new_content.as_bytes())
        .expect("failed to append line");
}

pub fn insert_line(path: &str, line_number: usize, new_content: &str) {
    // open file
    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);

    // create temp file to write to
    let new_file_path = generate_unique_tempname();
    let new_file = File::create(&new_file_path).expect("failed to create new file");
    let mut writer = BufWriter::new(new_file);

    // iterate over the lines and modify the desired line
    for (i, line) in reader.lines().enumerate() {
        let mut line = line.expect("failed to read line");
        line.push('\n');

        if i + 1 == line_number {
            let new_content = format!("{}{}", new_content, '\n');
            writer
                .write_all(new_content.as_bytes())
                .expect("failed to write line");
        }
        // copy the original line
        writer
            .write_all(line.as_bytes())
            .expect("failed to write line");
    }

    // flush the writer to ensure all data is written to the file
    writer.flush().expect("failed to flush");

    // replace the original file with the new file
    fs::rename(new_file_path, path).expect("failed to replace file");
}
