mod game_board;
mod game_result;
mod match_type;
mod word_chooser;

use std::io::stdin;

use match_type::MatchType;
use crate::game_board::GameBoard;
use crate::game_result::GameResult;
use crate::word_chooser::WordChooser;

fn main() {
    let mut word_chooser = WordChooser::new();
    let actual = word_chooser.choose_random_word();
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
    // TODO: Rename GameBoard to Game, have it keep new random word
    // TODO: Move random word choosing logic into WordChooser struct
    // TODO: Need to validate user input for length
    // TODO: Need to only allow for letters
    // TODO: Need to check that word is in dictionary
    // TODO: Need to return to user input if there are validation errors
    // TODO: Need to be able to start a new game (like with CTRL-N)
    // TODO: Need to be able to exit cleanly (like with CTRL-D)
}
