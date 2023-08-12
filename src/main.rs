use termion::color;
use termion::color::Color;
use std::iter::zip;

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

#[derive(PartialEq)]
enum MatchType {
    CorrectPosition,
    WrongPosition,
    None,
}

fn format_wordle_guess(guess: &str, actual: &str) -> String {
    let mut matchable_letters = actual.to_string();
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
            matchable_letters.remove(index);
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

    let mut formatted_result = "".to_string();
    for (guess_char, result) in zip(guess.chars(), results) {
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
    println!("{}", format_wordle_guess("pxppo", "abcpp"));
}
