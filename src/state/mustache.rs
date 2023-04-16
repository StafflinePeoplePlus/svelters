use super::{fragment::FragmentState, State, StateTransition};
use crate::{
    nodes::MustacheTag,
    parser::{ParseErrorKind, Parser},
    tokens::{MustacheCloseToken, MustacheOpenToken},
};
use swc_common::{source_map::BytePos, Spanned};
use swc_ecma_parser::{lexer::Lexer, StringInput, Syntax, TsConfig};

#[derive(Debug)]
pub struct MustacheState;

impl StateTransition for MustacheState {
    fn next_state(self, parser: &mut Parser<'_>) -> State {
        let start = parser.position();
        let mustache_open = MustacheOpenToken {
            span: parser
                .eat_char('{')
                .expect("should not enter state without moustache open"),
        };
        let leading_whitespace = parser.allow_whitespace();

        let source = parser.text();
        let mut ecma_parser = swc_ecma_parser::Parser::new_from(Lexer::new(
            Syntax::Typescript(TsConfig::default()),
            swc_ecma_ast::EsVersion::EsNext,
            StringInput::new(
                &source[parser.position()..],
                BytePos(parser.position() as u32),
                BytePos(source.len() as u32),
            ),
            None,
        ));

        let expression = ecma_parser.parse_expr().unwrap();
        parser.eat_to(expression.span_hi().0 as usize);
        let trailing_whitespace = parser.allow_whitespace();

        let mustache_close = match parser.eat_char('}') {
            Some(span) => Some(MustacheCloseToken { span }),
            None => {
                parser.emit_error(ParseErrorKind::MustacheNotClosed);
                None
            }
        };

        parser.push_node(MustacheTag {
            leading_whitespace,
            mustache_open,
            expression,
            mustache_close,
            trailing_whitespace,
            span: parser.span_from(start),
        });

        FragmentState.into()
    }
}
