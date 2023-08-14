use std::fs::read_to_string;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct WordChooser {
    rng: ThreadRng,
    words: Vec<String>,
}

impl WordChooser {
    pub fn new() -> Self {
        let rng = thread_rng();
        let words: Vec<String> = read_to_string("./words.txt")
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

        Self {
            rng,
            words,
        }
    }

    pub fn choose_random_word(&mut self) -> String {
        self.words.choose(&mut self.rng).unwrap().clone()
    }

    pub fn is_in_dictionary(&self, maybe_word: &str) -> bool {
        self.words.iter().any(|word| word == maybe_word)
    }
}