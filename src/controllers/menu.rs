use crate::controllers::{FileManager, KeyGenerator};

pub struct Menu;

const FILE_PATH: &str = "./generated_keys.txt";

impl Menu {
    pub fn new() -> Self {
        Self
    }

    // Starts menu
    pub fn menu_section(&self) {
        loop {
            println!("\n=== Menu ===");
            println!("1. Generate a new key");
            println!("2. Exit");
            print!("Select an option: ");

            // Get user option
            match Self::get_user_input().trim().parse::<i8>() {
                Ok(1) => self.generate_key(),
                Ok(2) => {
                    println!("Exiting program. Goodbye!");
                    break;
                }
                _ => println!("Invalid option. Please try again."),
            }
        }
    }

    // "Method" to capture the user input
    fn get_user_input() -> String {
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input
    }

    // Generate and store a new key
    fn generate_key(&self) {
        let generator = KeyGenerator::new();
        let result = generator.key_id();
        println!("Generated Key: {}", result);

        FileManager::add_key_to_file(FILE_PATH, &result);
        println!("Key has been stored in '{}'.", FILE_PATH);
    }
}
