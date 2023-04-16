use swc_common::{Span, Spanned};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseErrorKind {
    MustacheNotClosed,

    /// Missing whitespace after `{@debug`
    MissingWhitespaceAfterDebugTag,

    /// Missing whitespace after `{@const`
    MissingWhitespaceAfterConstTag,

    /// Missing whitespace after `{@html`
    MissingWhitespaceAfterHtmlTag,

    /// Expression given to `{@const ...}` was not a simple assignment expression
    InvalidConstArgs,

    /// One or more expressions given to `{@debug ...}` was not an identifier
    InvalidDebugArgs,
}

#[derive(Debug, Clone, Copy, Spanned, PartialEq)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub span: Span,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

pub trait ErrorReporter {
    fn report_parse_error(&mut self, error: ParseError);
}

#[derive(Default)]
pub struct CollectingErrorReporter {
    parse_errors: Vec<ParseError>,
}

impl CollectingErrorReporter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.parse_errors.is_empty()
    }

    pub fn parse_errors(&self) -> &[ParseError] {
        self.parse_errors.as_ref()
    }
}

impl ErrorReporter for CollectingErrorReporter {
    fn report_parse_error(&mut self, error: ParseError) {
        self.parse_errors.push(error);
    }
}
