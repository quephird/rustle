use termion::color;
use termion::color::Color;

pub trait HasCells {
    fn format_cell<C: Color>(&self, bg_color: C, letter: char) -> String {
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
