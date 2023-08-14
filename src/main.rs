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

fn main() {
    let mut game = Game::new();

    loop {
        game.display();
        println!("Enter a guess! ");
        let mut buffer = String::new();
        let _ignored = stdin().read_line(&mut buffer);
        let guess = buffer.trim();
        match game.validate_word(guess) {
            WordValidationResult::NotAllLetters => {
                println!("Word must be all letters!");
                continue;
            },
            WordValidationResult::NotFiveLetters => {
                println!("Word must be five letters long!");
                continue;
            },
            WordValidationResult::NotInDictionary => {
                println!("Word not in dictionary!");
                continue;
            },
            WordValidationResult::Ok => (),
        }

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
    // TODO: Need to validate user input for length
    // TODO: Need to only allow for letters
    // TODO: Need to check that word is in dictionary
    // TODO: Need to return to user input if there are validation errors
    // TODO: Need to be able to start a new game (like with CTRL-N)
    // TODO: Need to be able to exit cleanly (like with CTRL-D)
    // TODO: Think about introducing Guess and Keyboard as types
}
