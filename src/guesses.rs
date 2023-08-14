use crate::has_cells::HasCells;
use crate::letter_status::LetterStatus;

pub struct Guesses {
    current_guess_num: usize,
    guesses: [[(char, LetterStatus); 5]; 6],
}

impl HasCells for Guesses {}

impl Guesses {
    pub fn new() -> Self {
        Self {
            current_guess_num: 0,
            guesses: [[(' ', LetterStatus::NotGuessed); 5]; 6],
        }
    }

    pub fn submit_new_guess(&mut self, guess: [(char, LetterStatus); 5]) {
        self.guesses[self.current_guess_num] = guess;
        self.current_guess_num += 1;
    }

    pub fn get_guess_number(&self) -> usize {
        self.current_guess_num
    }

    pub fn display(&self) {
        for guess in self.guesses {
            let mut formatted_result = "".to_string();
            for (letter, status) in guess {
                let formatted_cell = self.format_cell(letter, status);
                formatted_result.push(' ');
                formatted_result.push_str(&formatted_cell);
            }

            println!("          {}\n", formatted_result);
        }
    }
}
