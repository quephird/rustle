use std::iter::zip;

use termion::color;
use termion::color::Color;

use crate::game_result::GameResult;
use crate::keyboard::Keyboard;
use crate::MatchType;
use crate::WordChooser;

pub struct Game {
    word_chooser: WordChooser,
    current_word: String,
    current_guess_num: usize,
    guesses: [[(char, MatchType); 5]; 6],
    keyboard: Keyboard,
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
            keyboard: Keyboard::new(),
        }
    }

    pub fn display(&self) {
        self.display_guesses();
        self.keyboard.display();
    }

    pub fn guess_word(&mut self, guess: &str) -> GameResult {
        // TODO: Need to think of a better name and document strategy below
        let mut matchable_letters = "".to_string();

        for (index, (guess_char, actual_char)) in zip(guess.chars(), self.current_word.chars()).enumerate() {
            if guess_char == actual_char {
                self.guesses[self.current_guess_num][index] = (guess_char, MatchType::CorrectPosition);
                self.keyboard.update_status(guess_char, MatchType::CorrectPosition);
            } else {
                matchable_letters.push(actual_char);
            }
        }

        for (guess_index, guess_char) in guess.chars().enumerate() {
            if self.guesses[self.current_guess_num][guess_index].1 == MatchType::CorrectPosition {
                continue;
            } else if let Some(match_index) = matchable_letters.find(guess_char) {
                self.guesses[self.current_guess_num][guess_index] = (guess_char, MatchType::WrongPosition);
                self.keyboard.update_status(guess_char, MatchType::WrongPosition);
                matchable_letters.remove(match_index);
            } else {
                self.guesses[self.current_guess_num][guess_index] = (guess_char, MatchType::None);
                self.keyboard.update_status(guess_char, MatchType::None);
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