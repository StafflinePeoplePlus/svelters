use swc_common::{ast_serde, EqIgnoreSpan, Span, Spanned};
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("CommentStartToken")]
pub struct CommentStartToken {
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("CommentEndToken")]
pub struct CommentEndToken {
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("MustacheOpenToken")]
pub struct MustacheOpenToken {
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("WhitespaceToken")]
pub struct WhitespaceToken {
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("MustacheCloseToken")]
pub struct MustacheCloseToken {
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("DebugTagToken")]
pub struct DebugTagToken {
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("ConstTagToken")]
pub struct ConstTagToken {
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("HtmlTagToken")]
pub struct HtmlTagToken {
    pub span: Span,
}
