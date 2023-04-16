use super::{mustache::MustacheState, tag::TagState, text::TextState, State, StateTransition};
use crate::parser::Parser;

#[derive(Debug)]
pub struct FragmentState;

impl StateTransition for FragmentState {
    fn next_state(self, parser: &mut Parser<'_>) -> State {
        match parser.peek() {
            Some('<') => TagState.into(),
            Some('{') => MustacheState.into(),
            Some(_) => TextState.into(),
            None => State::Eof,
        }
    }
}
