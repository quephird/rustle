#[derive(Clone, Copy, PartialEq)]
pub enum MatchType {
    CorrectPosition,
    WrongPosition,
    None,
    NotGuessed,
}
