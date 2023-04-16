use self::{fragment::FragmentState, mustache::MustacheState, tag::TagState, text::TextState};
use crate::parser::Parser;
use derive_more::From;

mod fragment;
mod mustache;
mod tag;
mod text;

#[derive(Debug, From)]
pub enum State {
    Fragment(FragmentState),
    Mustache(MustacheState),
    Tag(TagState),
    Text(TextState),
    Eof,
}

impl State {
    /// Returns `true` if the state is [`Eof`].
    ///
    /// [`Eof`]: State::Eof
    #[must_use]
    pub fn is_eof(&self) -> bool {
        matches!(self, Self::Eof)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Fragment(FragmentState)
    }
}

pub trait StateTransition {
    fn next_state(self, parser: &mut Parser<'_>) -> State;
}

impl StateTransition for State {
    fn next_state(self, parser: &mut Parser<'_>) -> State {
        match self {
            State::Fragment(s) => s.next_state(parser),
            State::Mustache(s) => s.next_state(parser),
            State::Tag(s) => s.next_state(parser),
            State::Text(s) => s.next_state(parser),
            State::Eof => State::Eof,
        }
    }
}
