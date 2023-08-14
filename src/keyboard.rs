use std::collections::HashMap;

use termion::color;

use crate::has_cells::HasCells;
use crate::letter_status::LetterStatus;

pub struct Keyboard {
    letter_statuses: HashMap<char, LetterStatus>,
}

impl HasCells for Keyboard {}

impl Keyboard {
    pub fn new() -> Self {
        let new_letter_statuses = ('a'..='z').fold(HashMap::new(), |mut acc, char| {
            acc.insert(char, LetterStatus::NotGuessed);
            acc
        });

        Self {
            letter_statuses: new_letter_statuses,
        }
    }

    pub fn get_status(&self, letter: char) -> &LetterStatus {
        self.letter_statuses.get(&letter).unwrap()
    }

    pub fn update_all_statuses(&mut self, latest_guess: [(char, LetterStatus); 5]) {
        for (guess_char, status) in latest_guess {
            self.update_status(guess_char, status);
        }
    }

    fn update_status(&mut self, letter: char, new_status: LetterStatus) {
        match new_status {
            LetterStatus::CorrectPosition => {
                self.letter_statuses.insert(letter, new_status);
            }
            LetterStatus::WrongPosition => {
                match self.get_status(letter) {
                    LetterStatus::NoMatch | LetterStatus::NotGuessed => {
                        self.letter_statuses.insert(letter, new_status);
                    },
                    _ => (),
                }
            },
            LetterStatus::NoMatch => {
                match self.get_status(letter) {
                    LetterStatus::NotGuessed => {
                        self.letter_statuses.insert(letter, new_status);
                    },
                    _ => (),
                }
            }
            LetterStatus::NotGuessed => (),
        }
    }

    pub fn display(&self) {
        for (indent, keyboard_row) in [("", "qwertyuiop"), (" ", "asdfghjkl"), ("  ", "zxcvbnm")] {
            let mut formatted_row = "".to_string();
            formatted_row.push_str(indent);
            for key_char in keyboard_row.chars() {
                if let Some(status) = self.letter_statuses.get(&key_char) {
                    let formatted_cell = match status {
                        LetterStatus::CorrectPosition => self.format_cell(color::Green, key_char),
                        LetterStatus::WrongPosition => self.format_cell(color::Yellow, key_char),
                        LetterStatus::NoMatch => self.format_cell(color::LightBlack, key_char),
                        LetterStatus::NotGuessed => self.format_cell(color::White, key_char),
                    };

                    formatted_row.push(' ');
                    formatted_row.push_str(&formatted_cell);
                }
            }

            println!("{}\n", formatted_row);
        }
    }
}
