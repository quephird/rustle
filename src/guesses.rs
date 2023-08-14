use termion::color;

use crate::has_cells::HasCells;
use crate::match_type::MatchType;

pub struct Guesses {
    guesses: [[(char, MatchType); 5]; 6],
}

impl HasCells for Guesses {}

impl Guesses {
    pub fn new() -> Self {
        Self {
            guesses: [[(' ', MatchType::NotGuessed); 5]; 6],
        }
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
