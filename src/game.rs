use std::collections::HashMap;
use std::iter::zip;

use termion::color;
use termion::color::Color;

use crate::game_result::GameResult;
use crate::{MatchType, WordChooser};

pub struct Game {
    word_chooser: WordChooser,
    current_word: String,
    current_guess_num: usize,
    guesses: [[(char, MatchType); 5]; 6],
    keyboard: HashMap<char, MatchType>,
}

impl Game {
    pub fn new() -> Self {
        let mut word_chooser = WordChooser::new();
        let new_word = word_chooser.choose_random_word();
        Self {
            word_chooser,
            current_word: new_word,
            current_guess_num: 0,
            guesses: Self::make_empty_guesses(),
            keyboard: Self::make_empty_keyboard(),
        }
    }

    pub fn display(&self) {
        self.display_guesses();
        self.display_keyboard();
    }

    pub fn guess_word(&mut self, guess: &str) -> GameResult {
        // TODO: Need to think of a better name and document strategy below
        let mut matchable_letters = "".to_string();

        for (index, (guess_char, actual_char)) in zip(guess.chars(), self.current_word.chars()).enumerate() {
            if guess_char == actual_char {
                self.guesses[self.current_guess_num][index] = (guess_char, MatchType::CorrectPosition);
                self.keyboard.insert(guess_char, MatchType::CorrectPosition);
            } else {
                matchable_letters.push(actual_char);
            }
        }

        for (guess_index, guess_char) in guess.chars().enumerate() {
            if self.guesses[self.current_guess_num][guess_index].1 == MatchType::CorrectPosition {
                continue;
            } else if let Some(match_index) = matchable_letters.find(guess_char) {
                self.guesses[self.current_guess_num][guess_index] = (guess_char, MatchType::WrongPosition);
                match self.keyboard.get(&guess_char) {
                    Some(MatchType::None) | Some(MatchType::NotGuessed) => {
                        self.keyboard.insert(guess_char, MatchType::WrongPosition);
                    },
                    _ => (),
                }
                matchable_letters.remove(match_index);
            } else {
                self.guesses[self.current_guess_num][guess_index] = (guess_char, MatchType::None);
                match self.keyboard.get(&guess_char) {
                    Some(MatchType::NotGuessed) => {
                        self.keyboard.insert(guess_char, MatchType::None);
                    },
                    _ => (),
                }
            }
        }

        if self.guesses[self.current_guess_num].iter().all(|(_, match_type)| *match_type == MatchType::CorrectPosition) {
            self.current_guess_num += 1;
            return GameResult::Win;
        } else if self.current_guess_num == 5 {
            return GameResult::Lose;
        } else {
            self.current_guess_num += 1;
            return GameResult::StillGoing;
        }
    }

    pub fn get_current_word(&self) -> &String {
        &self.current_word
    }

    fn make_empty_guesses() -> [[(char, MatchType); 5]; 6] {
        [[(' ', MatchType::NotGuessed); 5]; 6]
    }

    fn make_empty_keyboard() -> HashMap<char, MatchType> {
        ('a'..='z').fold(HashMap::new(), |mut acc, char| {
            acc.insert(char, MatchType::NotGuessed);
            acc
        })
    }

    fn display_guesses(&self) {
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

    fn display_keyboard(&self) {
        for (indent, keyboard_row) in [("", "qwertyuiop"), (" ", "asdfghjkl"), ("  ", "zxcvbnm")] {
            let mut formatted_row = "".to_string();
            formatted_row.push_str(indent);
            for key_char in keyboard_row.chars() {
                if let Some(match_type) = self.keyboard.get(&key_char) {
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

    fn format_cell<C: Color>(&self, bg_color: C, letter: char) -> String {
        format!(
            "{}{} {} {}{}",
            color::Bg(bg_color),
            color::Fg(color::Black),
            letter,
            color::Fg(color::Reset),
            color::Bg(color::Reset),
        )
    }
}