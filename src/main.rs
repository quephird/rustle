mod game;
mod guess_result;
mod guesses;
mod has_cells;
mod keyboard;
mod letter_status;
mod prompt_result;
mod word_chooser;
mod word_validation_result;

use crate::game::Game;

fn main() {
    let mut game = Game::new();

    loop {
        game.display();
        let guess = game.get_word_from_user();
        let guess_result = game.guess_word(guess);
        game.handle_guess_result(guess_result);
    }
}
