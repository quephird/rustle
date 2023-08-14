use termion::color;
use termion::color::Color;

use crate::letter_status::LetterStatus;

pub trait HasCells {
    fn format_cell(&self, letter: char, status: LetterStatus) -> String {
        match status {
            LetterStatus::CorrectPosition => self.format_cell_helper(color::Green, letter),
            LetterStatus::WrongPosition => self.format_cell_helper(color::Yellow, letter),
            LetterStatus::NoMatch => self.format_cell_helper(color::LightBlack, letter),
            LetterStatus::NotGuessed => self.format_cell_helper(color::White, letter),
        }
    }

    fn format_cell_helper<C: Color>(&self, bg_color: C, letter: char) -> String {
        format!(
            "{}{} {} {}{}",
            color::Bg(bg_color),
            color::Fg(color::Black),
            letter,
            color::Fg(color::Reset),
            color::Bg(color::Reset),
        )
    }
}
