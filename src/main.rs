use std::fs::read_to_string;
use std::io::stdin;
use std::iter::zip;
use std::process::exit;

use rand::seq::SliceRandom;
use rand::thread_rng;
use termion::color;
use termion::color::Color;

#[derive(PartialEq)]
enum MatchType {
    CorrectPosition,
    WrongPosition,
    None,
}

fn check_guess(guess: &str, actual: &str) -> [MatchType; 5] {
    // TODO: Need to think of a better name and document strategy below
    let mut matchable_letters = "".to_string();
    let mut results = [
        MatchType::None,
        MatchType::None,
        MatchType::None,
        MatchType::None,
        MatchType::None,
    ];

    for (index, (guess_char, actual_char)) in zip(guess.chars(), actual.chars()).enumerate() {
        if guess_char == actual_char {
            results[index] = MatchType::CorrectPosition;
        } else {
            matchable_letters.push(actual_char);
        }
    }

    for (guess_index, guess_char) in guess.chars().enumerate() {
        if results[guess_index] == MatchType::CorrectPosition {
            continue;
        } else if let Some(match_index) = matchable_letters.find(guess_char) {
            results[guess_index] = MatchType::WrongPosition;
            matchable_letters.remove(match_index);
        }
    }

    results
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

fn format_results(guess: &str, results: &[MatchType; 5]) -> String {
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
    formatted_result
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

    loop {
        println!("Enter a guess! ");
        let mut buffer = String::new();
        stdin().read_line(&mut buffer);
        let guess = buffer.trim();

        let results = check_guess(guess, actual);
        let formatted_results = format_results(guess, &results);
        println!("{}", formatted_results);

        if results.iter().all(|result| *result == MatchType::CorrectPosition) {
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
