mod game;
mod guess_result;
mod has_cells;
mod keyboard;
mod match_type;
mod word_chooser;
mod word_validation_result;

use std::io::stdin;

use match_type::MatchType;
use crate::game::Game;
use crate::guess_result::GuessResult;
use crate::word_chooser::WordChooser;
use crate::word_validation_result::WordValidationResult;

fn get_word_from_user(game: &Game) -> String {
    loop {
        println!("Enter a guess! ");
        let mut buffer = String::new();
        let _ignored = stdin().read_line(&mut buffer);
        let maybe_word = buffer.trim();
        match game.validate_word(maybe_word) {
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

fn main() {
    let mut game = Game::new();

    loop {
        game.display();
        let guess = get_word_from_user(&game);
        let result = game.guess_word(guess);
        match result {
            GuessResult::Win => {
                game.display();
                println!("You win!!!");
                break;
            },
            GuessResult::Lose => {
                game.display();
                println!("You lose :(");
                println!("The word was: {}", game.get_current_word());
                break;
            }
            GuessResult::StillGoing => (),
        }
    }
    // TODO: Need to be able to start a new game (like with CTRL-N)
    // TODO: Need to be able to exit cleanly (like with CTRL-D)
    // TODO: Think about introducing Guess and Keyboard as types
}
