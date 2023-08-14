mod game_board;
mod match_type;
mod game_result;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::stdin;
use std::iter::zip;

use rand::seq::SliceRandom;
use rand::thread_rng;
use termion::color;
use termion::color::Color;

use match_type::MatchType;
use crate::game_board::GameBoard;
use crate::game_result::GameResult;

fn main() {
    let words: Vec<String> = read_to_string("./words.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut rng = thread_rng();
    let actual = words.choose(&mut rng).unwrap();
    let mut game_board = GameBoard::new();

    loop {
        game_board.display();
        println!("Enter a guess! ");
        let mut buffer = String::new();
        let _ignored = stdin().read_line(&mut buffer);
        let guess = buffer.trim();

        let result = game_board.guess_word(guess, actual);
        match result {
            GameResult::Win => {
                game_board.display();
                println!("You win!!!");
                break;
            },
            GameResult::Lose => {
                game_board.display();
                println!("You lose :(");
                println!("The word was: {}", actual);
                break;
            }
            GameResult::StillGoing => (),
        }
    }
    // TODO: Need to validate user input for length
    // TODO: Need to only allow for letters
    // TODO: Need to check that word is in dictionary
    // TODO: Need to return to user input if there are validation errors
    // TODO: Need to be able to start a new game (like with CTRL-N)
    // TODO: Need to be able to exit cleanly (like with CTRL-D)
}
