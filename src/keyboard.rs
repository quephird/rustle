use std::collections::HashMap;

use termion::color;

use crate::has_cells::HasCells;
use crate::match_type::MatchType;

pub struct Keyboard {
    letter_statuses: HashMap<char, MatchType>,
}

impl HasCells for Keyboard {}

impl Keyboard {
    pub fn new() -> Self {
        let new_letter_statuses = ('a'..='z').fold(HashMap::new(), |mut acc, char| {
            acc.insert(char, MatchType::NotGuessed);
            acc
        });

        Self {
            letter_statuses: new_letter_statuses,
        }
    }

    pub fn get_status(&self, letter: char) -> &MatchType {
        self.letter_statuses.get(&letter).unwrap()
    }

    pub fn update_all_statuses(&mut self, latest_guess: [(char, MatchType); 5]) {
        for (guess_char, match_type) in latest_guess {
            self.update_status(guess_char, match_type);
        }
    }

    fn update_status(&mut self, letter: char, new_status: MatchType) {
        match new_status {
            MatchType::CorrectPosition => {
                self.letter_statuses.insert(letter, new_status);
            }
            MatchType::WrongPosition => {
                match self.get_status(letter) {
                    MatchType::None | MatchType::NotGuessed => {
                        self.letter_statuses.insert(letter, new_status);
                    },
                    _ => (),
                }
            },
            MatchType::None => {
                match self.get_status(letter) {
                    MatchType::NotGuessed => {
                        self.letter_statuses.insert(letter, new_status);
                    },
                    _ => (),
                }
            }
            MatchType::NotGuessed => (),
        }
    }

    pub fn display(&self) {
        for (indent, keyboard_row) in [("", "qwertyuiop"), (" ", "asdfghjkl"), ("  ", "zxcvbnm")] {
            let mut formatted_row = "".to_string();
            formatted_row.push_str(indent);
            for key_char in keyboard_row.chars() {
                if let Some(match_type) = self.letter_statuses.get(&key_char) {
                    let formatted_cell = match match_type {
                        MatchType::CorrectPosition => self.format_cell(color::Green, key_char),
                        MatchType::WrongPosition => self.format_cell(color::Yellow, key_char),
                        MatchType::None => self.format_cell(color::LightBlack, key_char),
                        MatchType::NotGuessed => self.format_cell(color::White, key_char),
                    };

                    formatted_row.push(' ');
                    formatted_row.push_str(&formatted_cell);
                }
            }

            println!("{}\n", formatted_row);
        }
    }
}
