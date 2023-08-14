mod game;
mod game_result;
mod match_type;
mod word_chooser;

use std::io::stdin;

use match_type::MatchType;
use crate::game::Game;
use crate::game_result::GameResult;
use crate::word_chooser::WordChooser;

fn main() {
    let mut game = Game::new();

    loop {
        game.display();
        println!("Enter a guess! ");
        let mut buffer = String::new();
        let _ignored = stdin().read_line(&mut buffer);
        let guess = buffer.trim();

        let result = game.guess_word(guess);
        match result {
            GameResult::Win => {
                game.display();
                println!("You win!!!");
                break;
            },
            GameResult::Lose => {
                game.display();
                println!("You lose :(");
                println!("The word was: {}", game.get_current_word());
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
    // TODO: Think about introducing Guess and Keyboard as types
}
