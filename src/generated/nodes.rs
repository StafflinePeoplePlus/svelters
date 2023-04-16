use super::tokens::*;
use derive_more::From;
use serde::{Deserialize, Serialize};
use swc_common::{ast_serde, EqIgnoreSpan, Span, Spanned};
#[derive(Debug, From, Spanned, Serialize, Deserialize, EqIgnoreSpan, PartialEq)]
#[serde(untagged)]
pub enum Node {
    Fragment(Fragment),
    FragmentItem(FragmentItem),
    Comment(Comment),
    Mustache(Mustache),
    Text(Text),
    CommentText(CommentText),
    MustacheItem(MustacheItem),
    DebugTag(DebugTag),
    ConstTag(ConstTag),
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("Fragment")]
pub struct Fragment {
    pub fragment_items: Vec<FragmentItem>,
    pub span: Span,
}
#[derive(Debug, Spanned, Serialize, Deserialize, EqIgnoreSpan, PartialEq, From)]
#[serde(untagged)]
pub enum FragmentItem {
    Comment(Comment),
    Mustache(Mustache),
    Text(Text),
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
#[ast_serde("Mustache")]
pub struct Mustache {
    pub mustache_open: MustacheOpenToken,
    pub leading_whitespace: Option<WhitespaceToken>,
    pub mustache_item: MustacheItem,
    pub trailing_whitespace: Option<WhitespaceToken>,
    pub mustache_close: Option<MustacheCloseToken>,
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("Text")]
pub struct Text {
    pub text: String,
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("CommentText")]
pub struct CommentText {
    pub text: String,
    pub span: Span,
}
#[derive(Debug, Spanned, Serialize, Deserialize, EqIgnoreSpan, PartialEq, From)]
#[serde(untagged)]
pub enum MustacheItem {
    DebugTag(DebugTag),
    ConstTag(ConstTag),
    Expression(Box<swc_ecma_ast::Expr>),
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("DebugTag")]
pub struct DebugTag {
    pub debug_tag: DebugTagToken,
    pub whitespace: Option<WhitespaceToken>,
    pub identifiers: Vec<Box<swc_ecma_ast::Expr>>,
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("ConstTag")]
pub struct ConstTag {
    pub const_tag: ConstTagToken,
    pub whitespace: WhitespaceToken,
    pub expression: Box<swc_ecma_ast::Expr>,
    pub span: Span,
}
