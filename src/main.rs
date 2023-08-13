mod word_results;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::stdin;
use std::iter::zip;

use rand::seq::SliceRandom;
use rand::thread_rng;
use termion::color;
use termion::color::Color;

#[derive(Clone, PartialEq)]
pub enum MatchType {
    CorrectPosition,
    WrongPosition,
    None,
}

pub struct WordResults {
    pub matches: [MatchType; 5],
    pub keyboard: HashMap<char, MatchType>,
}

impl WordResults {
    fn from(matches: [MatchType; 5], keyboard: HashMap<char, MatchType>) -> Self {
        Self {matches, keyboard}
    }
}

fn check_guess(guess: &str, actual: &str, keyboard: &mut HashMap<char, MatchType>) -> WordResults {
    // TODO: Need to think of a better name and document strategy below
    let mut matchable_letters = "".to_string();
    let mut matches = [
        MatchType::None,
        MatchType::None,
        MatchType::None,
        MatchType::None,
        MatchType::None,
    ];

    for (index, (guess_char, actual_char)) in zip(guess.chars(), actual.chars()).enumerate() {
        if guess_char == actual_char {
            matches[index] = MatchType::CorrectPosition;
            keyboard.insert(guess_char, MatchType::CorrectPosition);
        } else {
            matchable_letters.push(actual_char);
        }
    }

    for (guess_index, guess_char) in guess.chars().enumerate() {
        if matches[guess_index] == MatchType::CorrectPosition {
            continue;
        } else if let Some(match_index) = matchable_letters.find(guess_char) {
            matches[guess_index] = MatchType::WrongPosition;
            if keyboard.get(&guess_char).is_none() {
                keyboard.insert(guess_char, MatchType::WrongPosition);
            }
            matchable_letters.remove(match_index);
        }
    }

    WordResults::from(matches, keyboard.clone())
}

fn format_cell<C: Color>(bg_color: C, guess_char: char) -> String {
    format!(
        "{}{} {} {}{}",
        color::Bg(bg_color),
        color::Fg(color::Black),
        guess_char,
        color::Fg(color::Reset),
        color::Bg(color::Reset),
    )
}

fn display_matches(guess: &str, results: &[MatchType; 5]) {
    let mut formatted_result = "".to_string();
    for (guess_char, result) in zip(guess.to_ascii_uppercase().chars(), results) {
        let formatted_cell = match result {
            MatchType::CorrectPosition => format_cell(color::Green, guess_char),
            MatchType::WrongPosition => format_cell(color::Yellow, guess_char),
            MatchType::None => format_cell(color::LightBlack, guess_char),
        };

        formatted_result.push(' ');
        formatted_result.push_str(&formatted_cell);

    }

    println!("{}", formatted_result);
}

fn make_empty_keyboard() -> HashMap<char, MatchType> {
    ('a'..='z').fold(HashMap::new(), |mut acc, char| {
        acc.insert(char, MatchType::None);
        acc
    })
}

fn main() {
    let words: Vec<String> = read_to_string("./words.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut rng = thread_rng();
    let actual = words.choose(&mut rng).unwrap();
    let mut guesses = 1;
    let mut keyboard = make_empty_keyboard();

    loop {
        println!("Enter a guess! ");
        let mut buffer = String::new();
        stdin().read_line(&mut buffer);
        let guess = buffer.trim();

        let results = check_guess(guess, actual, &mut keyboard);
        display_matches(guess, &results.matches);

        if results.matches.iter().all(|result| *result == MatchType::CorrectPosition) {
            println!("You win!!!");
            break;
        } else if guesses == 6 {
            println!("You lose :(");
            println!("The word was: {}", actual);
            break;
        } else {
            guesses += 1;
        }
    }
    // TODO: Need to validate user input for length
    // TODO: Need to check that word is in dictionary
    // TODO: Need to return to user input if there are validation errors
    // TODO: Need to be able to start a new game (like with CTRL-N)
    // TODO: Need to be able to exit cleanly (like with CTRL-D)
    // TODO: Need to print the keyboard!!!
}
