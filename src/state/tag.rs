use super::{State, StateTransition};
use crate::{
    nodes::{Comment, CommentText},
    parser::Parser,
    tokens::{CommentEndToken, CommentStartToken},
};

#[derive(Debug)]
pub struct TagState;

impl StateTransition for TagState {
    fn next_state(self, parser: &mut Parser<'_>) -> State {
        let start = parser.position();

        if let Some(start_span) = parser.eat_chars("<!--") {
            let text_span = parser.eat_until_chars("-->");
            let end_span = parser.eat_chars("-->");
            let span = parser.span_from(start);

            parser.push_node(Comment {
                comment_start: CommentStartToken { span: start_span },
                comment_text: CommentText {
                    text: parser.text_span(&text_span).to_string(),
                    span: text_span,
                },
                comment_end: end_span.map(|span| CommentEndToken { span }),
                span,
            });

            return State::default();
        }

        todo!()
    }
}
