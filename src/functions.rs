use std::{
    fs::{self, File},
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
