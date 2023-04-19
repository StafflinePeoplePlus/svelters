use super::{fragment::FragmentState, State, StateTransition};
use crate::{
    error::ParseErrorKind,
    parser::Parser,
    syntax_nodes::{
        ConstTag, Context, DebugTag, EachAs, EachBlockOpen, EachIndex, Identifier, IfBlockOpen,
        InvalidSyntax, KeyBlockOpen, Mustache, MustacheItem, RawMustacheTag,
    },
    tokens::{
        ConstTagToken, DebugTagToken, HtmlTagToken, IfOpenToken, KeyOpenToken, MustacheCloseToken,
        MustacheOpenToken,
    },
};
use swc_common::{source_map::BytePos, Span, Spanned};
use swc_ecma_ast::{AssignOp, EsVersion, Expr, Ident};
use swc_ecma_parser::{lexer::Lexer, EsConfig, StringInput, Syntax};

#[derive(Debug, Default)]
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

        let mustache_item = if let Some(span) = parser.eat_char('#') {
            self.parse_block_open_tag(parser, span)
        } else if let Some(span) = parser.eat_chars("@html") {
            self.parse_raw_mustache_tag(parser, HtmlTagToken { span })
        } else if let Some(span) = parser.eat_chars("@debug") {
            self.parse_debug_tag(parser, DebugTagToken { span })
        } else if let Some(span) = parser.eat_chars("@const") {
            self.parse_const_tag(parser, ConstTagToken { span })
        } else {
            self.parse_mustache_tag(parser)
        };

        let trailing_whitespace = parser.allow_whitespace();

        let mustache_close = match parser.eat_char('}') {
            Some(span) => Some(MustacheCloseToken { span }),
            None => {
                parser.error(ParseErrorKind::MustacheNotClosed);
                None
            }
        };

        parser.push_node(Mustache {
            mustache_open,
            leading_whitespace,
            mustache_item,
            trailing_whitespace,
            mustache_close,
            span: parser.span_from(start),
        });
        FragmentState.into()
    }
}

impl MustacheState {
    fn parse_js_expression(&self, parser: &mut Parser<'_>) -> Box<Expr> {
        let source = parser.text();
        let mut ecma_parser = swc_ecma_parser::Parser::new_from(Lexer::new(
            Syntax::Es(EsConfig::default()),
            EsVersion::EsNext,
            StringInput::new(
                &source[parser.position()..],
                BytePos(parser.position() as u32),
                BytePos(source.len() as u32),
            ),
            None,
        ));

        let expression = ecma_parser.parse_expr().unwrap();
        parser.eat_to(expression.span_hi().0 as usize);
        expression
    }

    fn parse_mustache_tag(self, parser: &mut Parser<'_>) -> MustacheItem {
        self.parse_js_expression(parser).into()
    }

    fn parse_const_tag(self, parser: &mut Parser<'_>, const_tag: ConstTagToken) -> MustacheItem {
        let whitespace = parser
            .require_whitespace(ParseErrorKind::MissingWhitespaceAfterConstTag)
            .unwrap();
        let expression = self.parse_js_expression(parser);

        if !matches!(&*expression, Expr::Assign(expr) if expr.op == AssignOp::Assign) {
            parser.error_with_span(ParseErrorKind::InvalidConstArgs, expression.span());
        }

        let span = const_tag.span().with_hi(expression.span_hi());
        ConstTag {
            const_tag,
            whitespace,
            expression,
            span,
        }
        .into()
    }

    fn parse_debug_tag(self, parser: &mut Parser<'_>, debug_tag: DebugTagToken) -> MustacheItem {
        let whitespace = parser.allow_whitespace();

        let is_debug_all = matches!(parser.peek(), Some('}'));
        if !is_debug_all && whitespace.is_none() {
            todo!()
        }

        if is_debug_all {
            let span = match &whitespace {
                Some(ws) => debug_tag.span().with_hi(ws.span_hi()),
                None => debug_tag.span(),
            };

            DebugTag {
                debug_tag,
                whitespace,
                identifiers: Default::default(),
                span,
            }
            .into()
        } else {
            let expression = self.parse_js_expression(parser);
            let span = debug_tag.span().with_hi(expression.span_hi());

            // We don't match here as we don't want to have to rebox the expression if its just one
            let identifiers = if expression.is_seq() {
                expression.expect_seq().exprs
            } else {
                vec![expression]
            };
            for expression in &identifiers {
                if !matches!(&**expression, Expr::Ident(..)) {
                    parser.error_with_span(ParseErrorKind::InvalidDebugArgs, expression.span());
                }
            }

            DebugTag {
                debug_tag,
                whitespace,
                identifiers,
                span,
            }
            .into()
        }
    }

    fn parse_raw_mustache_tag(
        self,
        parser: &mut Parser<'_>,
        html_tag: HtmlTagToken,
    ) -> MustacheItem {
        let whitespace = parser
            .require_whitespace(ParseErrorKind::MissingWhitespaceAfterHtmlTag)
            .unwrap();
        let expression = self.parse_js_expression(parser);

        let span = html_tag.span().with_hi(expression.span_hi());
        RawMustacheTag {
            html_tag,
            whitespace,
            expression,
            span,
        }
        .into()
    }

    fn parse_block_open_tag(self, parser: &mut Parser<'_>, hash_span: Span) -> MustacheItem {
        if let Some(span) = parser.eat_chars("if") {
            let span = span.with_lo(hash_span.lo());
            let whitespace = parser
                .require_whitespace(ParseErrorKind::MissingWhitespaceAfterBlockOpen)
                .unwrap();
            let expression = self.parse_js_expression(parser);
            IfBlockOpen {
                if_open: IfOpenToken { span },
                whitespace,
                span: span.with_hi(expression.span_hi()),
                expression,
            }
            .into()
        } else if let Some(each_span) = parser.eat_chars("each") {
            let start = hash_span.lo().0 as usize;
            let each_span = each_span.with_lo(hash_span.lo());
            let whitespace = parser
                .require_whitespace(ParseErrorKind::MissingWhitespaceAfterBlockOpen)
                .unwrap();
            let expression = self.parse_js_expression(parser);
            let as_leading_ws = parser
                .require_whitespace(ParseErrorKind::MissingWhitespaceBeforeAs)
                .unwrap();
            let as_ = parser.eat_chars("as").unwrap().into();
            let as_trailing_ws = parser
                .require_whitespace(ParseErrorKind::MissingWhitespaceAfterAs)
                .unwrap();
            let context: Context = if Ident::is_valid_start(*parser.peek().unwrap()) {
                self.parse_identifier(parser).into()
            } else {
                todo!("destructuring")
            };

            let (index, key) = match parser.peek_ignore_whitespace() {
                Some(',') => (self.parse_each_index(parser), self.parse_each_key(parser)),
                Some('(') => (None, self.parse_each_key(parser)),
                _ => (None, None),
            };

            EachBlockOpen {
                each_open: each_span.into(),
                whitespace,
                expression,
                as_: EachAs {
                    span: as_leading_ws.span().with_hi(as_trailing_ws.span_hi()),
                    leading_ws: as_leading_ws,
                    as_,
                    trailing_ws: as_trailing_ws,
                },
                context,
                index,
                key,
                span: parser.span_from(start),
            }
            .into()
        } else if let Some(_span) = parser.eat_chars("await") {
            todo!()
        } else if let Some(span) = parser.eat_chars("key") {
            let span = span.with_lo(hash_span.lo());
            let whitespace = parser
                .require_whitespace(ParseErrorKind::MissingWhitespaceAfterBlockOpen)
                .unwrap();
            let expression = self.parse_js_expression(parser);
            KeyBlockOpen {
                key_open: KeyOpenToken { span },
                whitespace,
                span: span.with_hi(expression.span_hi()),
                expression,
            }
            .into()
        } else {
            let span = parser.eat_until_chars("}").with_lo(hash_span.lo());
            parser.error_with_span(ParseErrorKind::UnexpectedBlockType, span);
            InvalidSyntax {
                text: parser.text_span(&span).to_string(),
                span,
            }
            .into()
        }
    }

    fn parse_each_index(&self, parser: &mut Parser) -> Option<EachIndex> {
        let start = parser.position();
        let trailing_ws = parser.allow_whitespace();
        let comma = parser.eat_char(',')?;
        let whitespace = parser.allow_whitespace();
        let identifier = if Ident::is_valid_start(*parser.peek().unwrap()) {
            self.parse_identifier(parser)
        } else {
            todo!() // Throw some kind of parser error then attempt to recover.
        };

        Some(EachIndex {
            trailing_ws,
            comma: comma.into(),
            whitespace,
            identifier,
            span: parser.span_from(start),
        })
    }

    fn parse_each_key(&self, parser: &mut Parser) -> Option<crate::syntax_nodes::EachKey> {
        let _open_bracket = parser.eat_char('(')?;

        todo!("each key")
    }

    fn parse_identifier(&self, parser: &mut Parser) -> Identifier {
        let start = parser.position();
        parser.eat();
        let span = parser
            .eat_until(|c| !Ident::is_valid_continue(*c))
            .with_lo(BytePos(start as u32));
        Identifier {
            name: parser.text_span(&span).into(),
            span,
        }
    }
}
