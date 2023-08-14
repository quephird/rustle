use termion::color;

use crate::has_cells::HasCells;
use crate::match_type::MatchType;

pub struct Guesses {
    current_guess_num: usize,
    guesses: [[(char, MatchType); 5]; 6],
}

impl HasCells for Guesses {}

impl Guesses {
    pub fn new() -> Self {
        Self {
            current_guess_num: 0,
            guesses: [[(' ', MatchType::NotGuessed); 5]; 6],
        }
    }

    pub fn submit_new_guess(&mut self, guess: [(char, MatchType); 5]) {
        self.guesses[self.current_guess_num] = guess;
        self.current_guess_num += 1;
    }

    pub fn get_guess_number(&self) -> usize {
        self.current_guess_num
    }

    pub fn display(&self) {
        for guess in self.guesses {
            let mut formatted_result = "".to_string();
            for (guess_char, match_type) in guess {
                let formatted_cell = match match_type {
                    MatchType::CorrectPosition => self.format_cell(color::Green, guess_char),
                    MatchType::WrongPosition => self.format_cell(color::Yellow, guess_char),
                    MatchType::None => self.format_cell(color::LightBlack, guess_char),
                    MatchType::NotGuessed => self.format_cell(color::White, guess_char),
                };

                formatted_result.push(' ');
                formatted_result.push_str(&formatted_cell);
            }

            println!("          {}\n", formatted_result);
        }
    }
}
