mod controllers;

use controllers::{FileManager, KeyGenerator};

const FILE_PATH: &str = "./generated_keys.txt";

fn main() {
    // Create a new KeyGenerator
    let generator = KeyGenerator::new();

    // Generate a new key
    let result = generator.key_id();
    println!("Generated Key: {}", result);

    // Store the key on the file
    FileManager::add_key_to_file(FILE_PATH, &result);
}
