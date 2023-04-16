use swc_common::{ast_serde, EqIgnoreSpan, Span, Spanned};
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("MustacheOpen")]
pub struct MustacheOpenToken {
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("Whitespace")]
pub struct WhitespaceToken {
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("MustacheClose")]
pub struct MustacheCloseToken {
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("CommentStart")]
pub struct CommentStartToken {
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("CommentEnd")]
pub struct CommentEndToken {
    pub span: Span,
}
