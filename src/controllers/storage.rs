use std::fs::OpenOptions;
use std::io::Write;

pub fn add_key_to_file(file_path: &str, line: &str) {
    // Try to open the file to add the new generated key
    let result = OpenOptions::new()
        .create(true) // Create if not exists
        .append(true)
        .open(file_path)
        .and_then(|mut file| writeln!(file, "{}", line));

    match result {
        Ok(()) => println!("Line successfully added to '{}'", file_path),
        Err(e) => eprintln!("Error writing to the file: {}", e),
    }
}
