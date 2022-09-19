use std::iter::Peekable;

use crate::EditCommand;

pub fn parse_motion<'iter, I>(input: &mut Peekable<I>) -> Option<Motion>
where
    I: Iterator<Item = &'iter char>,
{
    match input.peek() {
        Some('b') => {
            let _ = input.next();
            Some(Motion::PreviousWord)
        }
        Some('B') => {
            let _ = input.next();
            Some(Motion::PreviousBigWord)
        }
        Some('w') => {
            let _ = input.next();
            Some(Motion::NextWord)
        }
        Some('W') => {
            let _ = input.next();
            Some(Motion::NextBigWord)
        }
        Some('e') => {
            let _ = input.next();
            Some(Motion::NextWordEnd)
        }
        Some('E') => {
            let _ = input.next();
            Some(Motion::NextBigWordEnd)
        }
        Some('d') => {
            let _ = input.next();
            Some(Motion::Line)
        }
        Some('0' | '^') => {
            let _ = input.next();
            Some(Motion::Start)
        }
        Some('$') => {
            let _ = input.next();
            Some(Motion::End)
        }
        Some('f') => {
            let _ = input.next();
            match input.peek() {
                Some(&x) => {
                    input.next();
                    Some(Motion::RightUntil(*x))
                }
                None => None,
            }
        }
        Some('t') => {
            let _ = input.next();
            match input.peek() {
                Some(&x) => {
                    input.next();
                    Some(Motion::RightBefore(*x))
                }
                None => None,
            }
        }
        Some('F') => {
            let _ = input.next();
            match input.peek() {
                Some(&x) => {
                    input.next();
                    Some(Motion::LeftUntil(*x))
                }
                None => None,
            }
        }
        Some('T') => {
            let _ = input.next();
            match input.peek() {
                Some(&x) => {
                    input.next();
                    Some(Motion::LeftBefore(*x))
                }
                None => None,
            }
        }
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Motion {
    NextWord,
    NextBigWord,
    NextWordEnd,
    NextBigWordEnd,
    PreviousWord,
    PreviousBigWord,
    Line,
    Start,
    End,
    RightUntil(char),
    RightBefore(char),
    LeftUntil(char),
    LeftBefore(char),
}

/// Vi left-right motions to or till a character.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ViToTill {
    /// f
    ToRight(char),
    /// F
    ToLeft(char),
    /// t
    TillRight(char),
    /// T
    TillLeft(char),
}

impl ViToTill {
    /// Swap the direction of the to or till for ','
    pub fn reverse(&self) -> Self {
        match self {
            ViToTill::ToRight(c) => ViToTill::ToLeft(*c),
            ViToTill::ToLeft(c) => ViToTill::ToRight(*c),
            ViToTill::TillRight(c) => ViToTill::TillLeft(*c),
            ViToTill::TillLeft(c) => ViToTill::TillRight(*c),
        }
    }
}

impl From<EditCommand> for Option<ViToTill> {
    fn from(edit: EditCommand) -> Self {
        match edit {
            EditCommand::MoveLeftBefore(c) => Some(ViToTill::TillLeft(c)),
            EditCommand::MoveLeftUntil(c) => Some(ViToTill::ToLeft(c)),
            EditCommand::MoveRightBefore(c) => Some(ViToTill::TillRight(c)),
            EditCommand::MoveRightUntil(c) => Some(ViToTill::ToRight(c)),
            _ => None,
        }
    }
}
