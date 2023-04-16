use super::{State, StateTransition};
use crate::{nodes::Text, parser::Parser, state::fragment::FragmentState};

#[derive(Debug)]
pub struct TextState;

impl StateTransition for TextState {
    fn next_state(self, parser: &mut Parser<'_>) -> State {
        let span = parser.eat_until(|c| matches!(c, '<' | '{'));
        parser.push_node(Text {
            text: parser.text_span(&span).to_string(),
            span,
        });

        FragmentState.into()
    }
}
