use std::io::stdin;
use std::iter::zip;

use crate::guess_result::GuessResult;
use crate::guesses::Guesses;
use crate::keyboard::Keyboard;
use crate::letter_status::LetterStatus;
use crate::word_validation_result::WordValidationResult;
use crate::word_chooser::WordChooser;

pub struct Game {
    word_chooser: WordChooser,
    current_word: String,
    guesses: Guesses,
    keyboard: Keyboard,
}

impl Game {
    pub fn new() -> Self {
        let mut word_chooser = WordChooser::new();
        let new_word = word_chooser.choose_random_word();
        Self {
            word_chooser,
            current_word: new_word,
            guesses: Guesses::new(),
            keyboard: Keyboard::new(),
        }
    }

    pub fn display(&self) {
        self.guesses.display();
        self.keyboard.display();
    }

    pub fn get_word_from_user(&self) -> String {
        loop {
            println!("Enter a guess! ");
            let mut buffer = String::new();
            let _ignored = stdin().read_line(&mut buffer);
            let maybe_word = buffer.trim();
            match self.validate_word(maybe_word) {
                WordValidationResult::NotAllLetters => {
                    println!("Word must be all letters!");
                },
                WordValidationResult::NotFiveLetters => {
                    println!("Word must be five letters long!");
                },
                WordValidationResult::NotInDictionary => {
                    println!("Word not in dictionary!");
                },
                WordValidationResult::Ok => return maybe_word.to_string(),
            }
        }
    }

    pub fn guess_word(&mut self, guess: String) -> GuessResult {
        // TODO: Need to think of a better name and document strategy below
        let mut matchable_letters = "".to_string();
        let mut new_guess_result = [(' ', LetterStatus::NotGuessed); 5];

        for (index, (guess_char, actual_char)) in zip(guess.chars(), self.current_word.chars()).enumerate() {
            if guess_char == actual_char {
                new_guess_result[index] = (guess_char, LetterStatus::CorrectPosition);
            } else {
                matchable_letters.push(actual_char);
            }
        }

        for (guess_index, guess_char) in guess.chars().enumerate() {
            if new_guess_result[guess_index].1 == LetterStatus::CorrectPosition {
                continue;
            } else if let Some(match_index) = matchable_letters.find(guess_char) {
                new_guess_result[guess_index] = (guess_char, LetterStatus::WrongPosition);
                matchable_letters.remove(match_index);
            } else {
                new_guess_result[guess_index] = (guess_char, LetterStatus::NoMatch);
            }
        }

        self.guesses.submit_new_guess(new_guess_result);
        self.keyboard.update_all_statuses(new_guess_result);

        if new_guess_result.iter().all(|(_, status)| *status == LetterStatus::CorrectPosition) {
            return GuessResult::Win;
        } else if self.guesses.get_guess_number() == 6 {
            return GuessResult::Lose;
        } else {
            return GuessResult::StillGoing;
        }
    }

    pub fn validate_word(&self, maybe_word: &str) -> WordValidationResult {
        if !maybe_word.chars().all(|c| c.is_ascii_alphabetic()) {
            return WordValidationResult::NotAllLetters;
        }
        if maybe_word.to_string().chars().count() != 5 {
            return WordValidationResult::NotFiveLetters;
        }
        if !self.word_chooser.is_in_dictionary(maybe_word) {
            return WordValidationResult::NotInDictionary;
        }

        WordValidationResult::Ok
    }

    pub fn get_current_word(&self) -> &String {
        &self.current_word
    }
}