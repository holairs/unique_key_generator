use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct KeyGenerator {
    numbers: Vec<i8>,
    symbols: Vec<&'static str>,
}

impl KeyGenerator {
    pub fn new() -> Self {
        Self {
            numbers: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            symbols: vec!["!", "@", "#", "$", "&", "^", "%", "?", "*", "b", "q", "w", "z", "a"],
        }
    }

    pub fn random_values(&self) -> (Vec<i8>, Vec<&'static str>) {
        let mut rng = thread_rng();
        let mut random_numbers = Vec::new();
        let mut random_symbols = Vec::new();

        for _ in 0..3 {
            if let Some(&num) = self.numbers.choose(&mut rng) {
                random_numbers.push(num);
            }
        }

        for _ in 0..3 {
            if let Some(&sym) = self.symbols.choose(&mut rng) {
                random_symbols.push(sym);
            }
        }

        (random_numbers, random_symbols)
    }

    pub fn generate_block(&self) -> String {
        let mut rng = thread_rng();
        let (mut random_numbers, mut random_symbols) = self.random_values();
        let mut mix: Vec<String> = Vec::new();

        mix.extend(random_numbers.drain(..).map(|n| n.to_string()));
        mix.extend(random_symbols.drain(..).map(|s| s.to_string()));

        mix.shuffle(&mut rng);
        mix.join("")
    }

    pub fn key_id(&self) -> String {
        (0..5)
            .map(|i| {
                let block = self.generate_block();
                if i < 4 {
                    format!("{}-", block)
                } else {
                    block
                }
            })
            .collect()
    }
}
