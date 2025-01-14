use controllers::generate_values::key_id;
use controllers::storage::add_key_to_file;

mod controllers;

const FILE_PATH: &str = "./generated_keys.txt";

fn main() {
    let result = key_id();
    println!("Generated Key: {}", result);

    // Store generated key on a txt file
    add_key_to_file(FILE_PATH, &result);
}
