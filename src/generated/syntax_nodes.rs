use super::tokens::*;
use derive_more::From;
use serde::{Deserialize, Serialize};
use swc_common::{ast_serde, EqIgnoreSpan, Span, Spanned};
#[derive(Debug, From, Spanned, Serialize, Deserialize, EqIgnoreSpan, PartialEq)]
#[serde(untagged)]
pub enum Node {
    Text(Text),
    InvalidSyntax(InvalidSyntax),
    Comment(Comment),
    CommentText(CommentText),
    Mustache(Mustache),
    MustacheItem(MustacheItem),
    BlockOpen(BlockOpen),
    BlockClose(BlockClose),
    RawMustacheTag(RawMustacheTag),
    DebugTag(DebugTag),
    ConstTag(ConstTag),
    IfBlockOpen(IfBlockOpen),
    EachBlockOpen(EachBlockOpen),
    KeyBlockOpen(KeyBlockOpen),
    EachAs(EachAs),
    EachIndex(EachIndex),
    EachKey(EachKey),
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("Text")]
pub struct Text {
    pub text: String,
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("InvalidSyntax")]
pub struct InvalidSyntax {
    pub text: String,
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
#[derive(Debug, Spanned, Serialize, Deserialize, EqIgnoreSpan, PartialEq, From)]
#[serde(untagged)]
pub enum MustacheItem {
    BlockOpen(BlockOpen),
    BlockClose(BlockClose),
    RawMustacheTag(RawMustacheTag),
    DebugTag(DebugTag),
    ConstTag(ConstTag),
    Expression(Box<swc_ecma_ast::Expr>),
    InvalidSyntax(InvalidSyntax),
}
#[derive(Debug, Spanned, Serialize, Deserialize, EqIgnoreSpan, PartialEq, From)]
#[serde(untagged)]
pub enum BlockOpen {
    IfBlockOpen(IfBlockOpen),
    EachBlockOpen(EachBlockOpen),
    KeyBlockOpen(KeyBlockOpen),
    Unknown(InvalidSyntax),
}
#[derive(Debug, Spanned, Serialize, Deserialize, EqIgnoreSpan, PartialEq, From)]
#[serde(untagged)]
pub enum BlockClose {
    IfClose(IfCloseToken),
    EachClose(EachCloseToken),
    AwaitClose(AwaitCloseToken),
    KeyClose(KeyCloseToken),
    Unknown(InvalidSyntax),
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("RawMustacheTag")]
pub struct RawMustacheTag {
    pub html_tag: HtmlTagToken,
    pub whitespace: WhitespaceToken,
    pub expression: Box<swc_ecma_ast::Expr>,
    pub span: Span,
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
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("IfBlockOpen")]
pub struct IfBlockOpen {
    pub if_open: IfOpenToken,
    pub whitespace: WhitespaceToken,
    pub expression: Box<swc_ecma_ast::Expr>,
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("EachBlockOpen")]
pub struct EachBlockOpen {
    pub each_open: EachOpenToken,
    pub whitespace: WhitespaceToken,
    pub expression: Box<swc_ecma_ast::Expr>,
    pub as_: EachAs,
    pub context: swc_ecma_ast::Pat,
    pub index: Option<EachIndex>,
    pub key: Option<EachKey>,
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("KeyBlockOpen")]
pub struct KeyBlockOpen {
    pub key_open: KeyOpenToken,
    pub whitespace: WhitespaceToken,
    pub expression: Box<swc_ecma_ast::Expr>,
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("EachAs")]
pub struct EachAs {
    pub leading_ws: WhitespaceToken,
    pub as_: AsToken,
    pub trailing_ws: WhitespaceToken,
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("EachIndex")]
pub struct EachIndex {
    pub trailing_ws: Option<WhitespaceToken>,
    pub comma: CommaToken,
    pub whitespace: Option<WhitespaceToken>,
    pub identifier: swc_ecma_ast::Ident,
    pub span: Span,
}
#[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
#[ast_serde("EachKey")]
pub struct EachKey {
    pub whitespace: Option<WhitespaceToken>,
    pub paren_open: ParenOpenToken,
    pub leading_ws: Option<WhitespaceToken>,
    pub expression: Box<swc_ecma_ast::Expr>,
    pub trailing_ws: Option<WhitespaceToken>,
    pub paren_close: ParenCloseToken,
    pub span: Span,
}
