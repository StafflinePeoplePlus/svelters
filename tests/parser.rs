use svelters::{
    nodes::{Comment, CommentText, MustacheTag, Text},
    parser::{new_span, Parser},
    tokens::{
        CommentEndToken, CommentStartToken, MustacheCloseToken, MustacheOpenToken, WhitespaceToken,
    },
};
use swc_ecma_ast::{Expr, Ident};

#[test]
fn fragment() {
    let nodes = Parser::new("Hello, {world}!").parse();
    assert_eq!(nodes.len(), 3);
}

#[test]
fn text() {
    let nodes = Parser::new("Hello, world!").parse();

    assert_eq!(
        nodes,
        vec![Text {
            text: "Hello, world!".into(),
            span: new_span(0, 13),
        }
        .into()]
    );
}

#[test]
fn mustache_tag() {
    let nodes = Parser::new("{hello}").parse();
    let expected_node = MustacheTag {
        mustache_open: MustacheOpenToken {
            span: new_span(0, 1),
        },
        leading_whitespace: None,
        expression: Box::new(Expr::Ident(Ident::new("hello".into(), new_span(1, 6)))),
        trailing_whitespace: None,
        mustache_close: Some(MustacheCloseToken {
            span: new_span(6, 7),
        }),
        span: new_span(0, 7),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
}

#[test]
fn mustache_tag_whitespace() {
    let nodes = Parser::new("{  hello   }").parse();
    let expected_node = MustacheTag {
        mustache_open: MustacheOpenToken {
            span: new_span(0, 1),
        },
        leading_whitespace: Some(WhitespaceToken {
            span: new_span(1, 3),
        }),
        expression: Box::new(Expr::Ident(Ident::new("hello".into(), new_span(3, 8)))),
        trailing_whitespace: Some(WhitespaceToken {
            span: new_span(8, 11),
        }),
        mustache_close: Some(MustacheCloseToken {
            span: new_span(11, 12),
        }),
        span: new_span(0, 12),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
}

#[test]
fn mustache_tag_missing_close() {
    let nodes = Parser::new("{hello").parse();
    let expected_node = MustacheTag {
        mustache_open: MustacheOpenToken {
            span: new_span(0, 1),
        },
        leading_whitespace: None,
        expression: Box::new(Expr::Ident(Ident::new("hello".into(), new_span(1, 6)))),
        trailing_whitespace: None,
        mustache_close: None,
        span: new_span(0, 6),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
}

#[test]
fn comment() {
    let nodes = Parser::new("<!-- a comment -->").parse();

    assert_eq!(
        nodes,
        vec![Comment {
            comment_start: CommentStartToken {
                span: new_span(0, 4)
            },
            comment_text: CommentText {
                text: " a comment ".into(),
                span: new_span(4, 15)
            },
            comment_end: Some(CommentEndToken {
                span: new_span(15, 18)
            }),
            span: new_span(0, 18),
        }
        .into()]
    );
}
