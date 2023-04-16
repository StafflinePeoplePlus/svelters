use super::tokens::*;
use derive_more::From;
use serde::{Deserialize, Serialize};
use swc_common::{ast_serde, EqIgnoreSpan, Span, Spanned};
#[derive(Debug, From, Spanned, Serialize, Deserialize, EqIgnoreSpan, PartialEq)]
#[serde(untagged)]
pub enum Node {
    Fragment(Fragment),
    FragmentItem(FragmentItem),
    Text(Text),
    MustacheTag(MustacheTag),
    Comment(Comment),
    CommentText(CommentText),
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("Fragment")]
pub struct Fragment {
    pub fragment_items: Vec<FragmentItem>,
    pub span: Span,
}
#[derive(Debug, Spanned, Serialize, Deserialize, EqIgnoreSpan, PartialEq)]
#[serde(untagged)]
pub enum FragmentItem {
    Text(Text),
    MustacheTag(MustacheTag),
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("Text")]
pub struct Text {
    pub text: String,
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("MustacheTag")]
pub struct MustacheTag {
    pub mustache_open: MustacheOpenToken,
    pub leading_whitespace: Option<WhitespaceToken>,
    pub expression: Box<swc_ecma_ast::Expr>,
    pub trailing_whitespace: Option<WhitespaceToken>,
    pub mustache_close: Option<MustacheCloseToken>,
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("Comment")]
pub struct Comment {
    pub comment_start: CommentStartToken,
    pub comment_text: CommentText,
    pub comment_end: Option<CommentEndToken>,
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("CommentText")]
pub struct CommentText {
    pub text: String,
    pub span: Span,
}
