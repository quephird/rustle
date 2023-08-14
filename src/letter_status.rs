#[derive(Clone, Copy, PartialEq)]
pub enum LetterStatus {
    CorrectPosition,
    WrongPosition,
    NoMatch,
    NotGuessed,
}
