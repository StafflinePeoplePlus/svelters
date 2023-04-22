use swc_common::{ast_serde, EqIgnoreSpan, Span, Spanned};
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("CommentStartToken")]
pub struct CommentStartToken {
    pub span: Span,
}
impl From<Span> for CommentStartToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("CommentEndToken")]
pub struct CommentEndToken {
    pub span: Span,
}
impl From<Span> for CommentEndToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("MustacheOpenToken")]
pub struct MustacheOpenToken {
    pub span: Span,
}
impl From<Span> for MustacheOpenToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("WhitespaceToken")]
pub struct WhitespaceToken {
    pub span: Span,
}
impl From<Span> for WhitespaceToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("MustacheCloseToken")]
pub struct MustacheCloseToken {
    pub span: Span,
}
impl From<Span> for MustacheCloseToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("DebugTagToken")]
pub struct DebugTagToken {
    pub span: Span,
}
impl From<Span> for DebugTagToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("ConstTagToken")]
pub struct ConstTagToken {
    pub span: Span,
}
impl From<Span> for ConstTagToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("HtmlTagToken")]
pub struct HtmlTagToken {
    pub span: Span,
}
impl From<Span> for HtmlTagToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("KeyOpenToken")]
pub struct KeyOpenToken {
    pub span: Span,
}
impl From<Span> for KeyOpenToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("IfOpenToken")]
pub struct IfOpenToken {
    pub span: Span,
}
impl From<Span> for IfOpenToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("EachOpenToken")]
pub struct EachOpenToken {
    pub span: Span,
}
impl From<Span> for EachOpenToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("AsToken")]
pub struct AsToken {
    pub span: Span,
}
impl From<Span> for AsToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("CommaToken")]
pub struct CommaToken {
    pub span: Span,
}
impl From<Span> for CommaToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("ParenOpenToken")]
pub struct ParenOpenToken {
    pub span: Span,
}
impl From<Span> for ParenOpenToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("ParenCloseToken")]
pub struct ParenCloseToken {
    pub span: Span,
}
impl From<Span> for ParenCloseToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("IfCloseToken")]
pub struct IfCloseToken {
    pub span: Span,
}
impl From<Span> for IfCloseToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("EachCloseToken")]
pub struct EachCloseToken {
    pub span: Span,
}
impl From<Span> for EachCloseToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("AwaitCloseToken")]
pub struct AwaitCloseToken {
    pub span: Span,
}
impl From<Span> for AwaitCloseToken {
    fn from(span: Span) -> Self { Self { span } }
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("KeyCloseToken")]
pub struct KeyCloseToken {
    pub span: Span,
}
impl From<Span> for KeyCloseToken {
    fn from(span: Span) -> Self { Self { span } }
}
