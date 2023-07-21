use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
};

fn main() {
    if env::args().len() < 2 {
        eprintln!("No file provided");
        return;
    }

    // 1-index based
    let line_to_modify = 6;
    let new_line_content = "this line was modified x2";

    // we know that is at least length 2, so its safe to unwrap
    let path = env::args().nth(1).unwrap();

    // open current file
    let file = File::open(&path).expect("failed to open file");
    let reader = BufReader::new(file);

    // create temp file to write to
    // TODO: make unique name for temp file
    let new_file_path = "temptemp";
    let new_file = File::create(new_file_path).expect("failed to create new file");
    let mut writer = BufWriter::new(new_file);

    // iterate over the lines and modify the desired line
    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("failed to read line");

        if i + 1 == line_to_modify {
            // modify desired line
            writer
                .write_all(new_line_content.as_bytes())
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
