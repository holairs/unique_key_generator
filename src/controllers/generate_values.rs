use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn random_values() -> (Vec<i8>, Vec<&'static str>) {
    let mut random_numbers: Vec<i8> = Vec::new();
    let mut random_symbols: Vec<&'static str> = Vec::new();
    let numbers: [i8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let symbols: [&str; 13] = [
        "!", "@", "#", "$", "&", "^", "%", "?", "*", "b", "q", "w", "z",
    ];

    let mut rng = thread_rng();

    // Select 4 random numbers
    for _ in 0..3 {
        if let Some(&random_number) = numbers.choose(&mut rng) {
            random_numbers.push(random_number);
        }
    }

    // Select 4 random symbols
    for _ in 0..3 {
        if let Some(&random_symbol) = symbols.choose(&mut rng) {
            random_symbols.push(random_symbol);
        }
    }

    // Return generated vectors
    (random_numbers, random_symbols)
}

pub fn generate_block() -> String {
    let mut rng = thread_rng();

    // Get random numbers and symbols
    let (mut random_numbers, mut random_symbols) = random_values();

    // Set all in a constant
    let mut mix: Vec<String> = Vec::new();

    // Add numbers to mix
    for number in random_numbers.drain(..) {
        mix.push(number.to_string());
    }
    // Add symbols to mix
    for symbol in random_symbols.drain(..) {
        mix.push(symbol.to_string());
    }

    // Randomize the mix
    mix.shuffle(&mut rng);

    // Join and return the new block
    mix.join("")
}

pub fn key_id() -> String {
    let mut result = String::new();

    // Generate 4 different blocks
    for i in 0..5 {
        let block = generate_block();
        result.push_str(&block);
        if i < 4 {
            result.push('-'); // Add a "-" inside each block
        }
    }

    result
}
