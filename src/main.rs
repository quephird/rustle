mod game;
mod guess_result;
mod has_cells;
mod keyboard;
mod match_type;
mod word_chooser;
mod word_validation_result;

use crate::game::Game;
use crate::guess_result::GuessResult;

fn main() {
    let mut game = Game::new();

    loop {
        game.display();
        let guess = game.get_word_from_user();
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
