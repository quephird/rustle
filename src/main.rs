mod game;
mod guess_result;
mod guesses;
mod has_cells;
mod keyboard;
mod letter_status;
mod prompt_result;
mod word_chooser;
mod word_validation_result;

use std::io::stdin;

use crate::game::Game;
use crate::guess_result::GuessResult;
use crate::prompt_result::PromptResult;

fn prompt_for_another_game() -> PromptResult {
    loop {
        println!("Did you want to play another game? (y/n)");
        let mut buffer = String::new();
        let _ignored = stdin().read_line(&mut buffer);
        let response = buffer.trim();
        match response.to_ascii_uppercase().as_str() {
            "Y" => return PromptResult::Yes,
            "N" => return PromptResult::No,
            _ => (),
        }
    }
}

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
                match prompt_for_another_game() {
                    PromptResult::Yes => {
                        println!("\nStarting a new game...\n");
                        game.reset();
                    },
                    PromptResult::No => {
                        break;
                    },
                }
            },
            GuessResult::Lose => {
                game.display();
                println!("You lose :(");
                println!("The word was: {}", game.get_current_word());
                match prompt_for_another_game() {
                    PromptResult::Yes => {
                        println!("\nStarting a new game...\n");
                        game.reset();
                    },
                    PromptResult::No => {
                        break;
                    },
                }
            }
            GuessResult::StillGoing => (),
        }
    }
}
