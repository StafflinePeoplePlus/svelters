use crate::{
    error::{ErrorReporter, ParseError, ParseErrorKind},
    nodes::Node,
    state::{State, StateTransition},
    tokens::WhitespaceToken,
};
use muncher::Muncher;
use swc_common::{BytePos, Span};

pub struct Parser<'a> {
    muncher: Muncher<'a>,
    nodes: Vec<Node>,
    error_reporter: &'a mut dyn ErrorReporter,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, error_reporter: &'a mut dyn ErrorReporter) -> Self {
        Self {
            muncher: Muncher::new(source),
            nodes: Default::default(),
            error_reporter,
        }
    }

    pub fn parse(mut self) -> Vec<Node> {
        let mut state = State::default();
        while !state.is_eof() {
            state = state.next_state(&mut self);
        }
        self.nodes
    }

    pub(crate) fn peek(&self) -> Option<&char> {
        self.muncher.peek()
    }

    pub(crate) fn push_node(&mut self, node: impl Into<Node>) {
        self.nodes.push(node.into())
    }

    pub(crate) fn eat(&mut self) -> Option<char> {
        self.muncher.eat()
    }

    pub(crate) fn eat_char(&mut self, c: char) -> Option<Span> {
        let start = self.muncher.position();
        self.muncher.reset_peek();
        match self.muncher.peek() {
            Some(x) if x == &c => {
                self.muncher.eat();
                Some(new_span(start, self.muncher.position()))
            }
            _ => {
                self.muncher.reset_peek();
                None
            }
        }
    }

    pub(crate) fn eat_chars(&mut self, s: &str) -> Option<Span> {
        let start = self.muncher.position();
        self.muncher.reset_peek();

        for c in s.chars() {
            match self.muncher.peek() {
                Some(x) if x == &c => {
                    self.muncher.eat();
                }
                _ => {
                    self.muncher.reset_peek();
                    return None;
                }
            }
        }

        Some(new_span(start, self.muncher.position()))
    }

    pub(crate) fn eat_to(&mut self, position: usize) {
        while self.muncher.position() < position {
            if self.eat().is_none() {
                break;
            }
        }
    }

    pub(crate) fn allow_whitespace(&mut self) -> Option<WhitespaceToken> {
        if !self.muncher.peek()?.is_whitespace() {
            return None;
        }
        let (start, end) = self.muncher.eat_until_count(|c| !c.is_ascii_whitespace());
        Some(WhitespaceToken {
            span: new_span(start, end),
        })
    }

    pub(crate) fn require_whitespace(&mut self, error: ParseErrorKind) -> Option<WhitespaceToken> {
        let whitespace = self.allow_whitespace();
        if whitespace.is_none() {
            self.error(error)
        }
        whitespace
    }

    pub(crate) fn eat_until<P>(&mut self, pred: P) -> Span
    where
        P: FnMut(&char) -> bool,
    {
        let (start, end) = self.muncher.eat_until_count(pred);
        new_span(start, end)
    }

    pub(crate) fn eat_until_chars(&mut self, s: &str) -> Span {
        let (start, end) = self.muncher.eat_range_of(s);
        new_span(start, end)
    }

    pub(crate) fn position(&self) -> usize {
        self.muncher.position()
    }

    pub(crate) fn text(&self) -> &'a str {
        self.muncher.text()
    }

    pub(crate) fn text_span(&self, span: &Span) -> &'a str {
        &self.muncher.text()[(span.lo.0 as usize)..(span.hi.0 as usize)]
    }

    pub(crate) fn error(&mut self, kind: ParseErrorKind) {
        let pos = self.muncher.position();
        self.error_with_span(kind, new_span(pos - 1, pos));
    }

    pub(crate) fn error_with_span(&mut self, kind: ParseErrorKind, span: Span) {
        self.error_reporter
            .report_parse_error(ParseError { kind, span });
    }

    pub(crate) fn span_from(&self, start: usize) -> Span {
        new_span(start, self.muncher.position())
    }
}

pub fn new_span(start: usize, end: usize) -> Span {
    Span::new(
        BytePos(start as u32),
        BytePos(end as u32),
        Default::default(),
    )
}
