use std::collections::HashMap;

use termion::color;

use crate::has_cells::HasCells;
use crate::MatchType;

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
