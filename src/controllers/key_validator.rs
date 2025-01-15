use std::fs::File;
use std::io::{self, BufRead};

pub struct KeyValidator;

impl KeyValidator {
    pub fn is_key_valid(file_path: &str, key: &str) -> bool {
        if let Ok(file) = File::open(file_path) {
            let reader = io::BufReader::new(file);

            for line in reader.lines() {
                if let Ok(existing_key) = line {
                    if existing_key.trim() == key.trim() {
                        return true;
                    }
                }
            }
        }

        // If file not exists or key is not on the file
        false
    }
}
