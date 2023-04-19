use svelters::{
    error::{CollectingErrorReporter, ParseError, ParseErrorKind},
    parser::{new_span, Parser},
    syntax_nodes::{
        Comment, CommentText, ConstTag, DebugTag, EachAs, EachBlockOpen, Identifier, IfBlockOpen,
        InvalidSyntax, KeyBlockOpen, Mustache, RawMustacheTag, Text,
    },
    tokens::{
        CommentEndToken, CommentStartToken, ConstTagToken, DebugTagToken, HtmlTagToken,
        IfOpenToken, KeyOpenToken, MustacheCloseToken, MustacheOpenToken, WhitespaceToken,
    },
};
use swc_ecma_ast::{Expr, Ident, Lit, Number};

#[test]
fn fragment() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("Hello, {world}!", &mut error_reporter).parse();
    assert_eq!(nodes.len(), 3);
    assert!(error_reporter.is_empty())
}

#[test]
fn text() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("Hello, world!", &mut error_reporter).parse();

    assert_eq!(
        nodes,
        vec![Text {
            text: "Hello, world!".into(),
            span: new_span(0, 13),
        }
        .into()]
    );
    assert!(error_reporter.is_empty())
}

#[test]
fn mustache_expression() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{hello}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: MustacheOpenToken {
            span: new_span(0, 1),
        },
        leading_whitespace: None,
        mustache_item: Box::new(Expr::Ident(Ident::new("hello".into(), new_span(1, 6)))).into(),
        trailing_whitespace: None,
        mustache_close: Some(MustacheCloseToken {
            span: new_span(6, 7),
        }),
        span: new_span(0, 7),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert!(error_reporter.is_empty())
}

#[test]
fn mustache_expression_whitespace() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{  hello   }", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: MustacheOpenToken {
            span: new_span(0, 1),
        },
        leading_whitespace: Some(WhitespaceToken {
            span: new_span(1, 3),
        }),
        mustache_item: Box::new(Expr::Ident(Ident::new("hello".into(), new_span(3, 8)))).into(),
        trailing_whitespace: Some(WhitespaceToken {
            span: new_span(8, 11),
        }),
        mustache_close: Some(MustacheCloseToken {
            span: new_span(11, 12),
        }),
        span: new_span(0, 12),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert!(error_reporter.is_empty())
}

#[test]
fn mustache_expression_missing_close() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{hello", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: MustacheOpenToken {
            span: new_span(0, 1),
        },
        leading_whitespace: None,
        mustache_item: Box::new(Expr::Ident(Ident::new("hello".into(), new_span(1, 6)))).into(),
        trailing_whitespace: None,
        mustache_close: None,
        span: new_span(0, 6),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert_eq!(
        error_reporter.parse_errors(),
        &[ParseError::new(
            ParseErrorKind::MustacheNotClosed,
            new_span(5, 6)
        )]
    );
}

#[test]
fn comment() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("<!-- a comment -->", &mut error_reporter).parse();

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
    assert!(error_reporter.is_empty())
}

#[test]
fn mustache_const_tag() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{@const hello}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: MustacheOpenToken {
            span: new_span(0, 1),
        },
        leading_whitespace: None,
        mustache_item: ConstTag {
            const_tag: ConstTagToken {
                span: new_span(1, 7),
            },
            whitespace: WhitespaceToken {
                span: new_span(7, 8),
            },
            expression: Box::new(Expr::Ident(Ident::new("hello".into(), new_span(8, 13)))),
            span: new_span(1, 13),
        }
        .into(),
        trailing_whitespace: None,
        mustache_close: Some(MustacheCloseToken {
            span: new_span(13, 14),
        }),
        span: new_span(0, 14),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert_eq!(
        error_reporter.parse_errors(),
        &[ParseError::new(
            ParseErrorKind::InvalidConstArgs,
            new_span(8, 13)
        )]
    );
}

#[test]
fn mustache_debug_tag() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{@debug hello}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: MustacheOpenToken {
            span: new_span(0, 1),
        },
        leading_whitespace: None,
        mustache_item: DebugTag {
            debug_tag: DebugTagToken {
                span: new_span(1, 7),
            },
            whitespace: Some(WhitespaceToken {
                span: new_span(7, 8),
            }),
            identifiers: vec![Box::new(Expr::Ident(Ident::new(
                "hello".into(),
                new_span(8, 13),
            )))],
            span: new_span(1, 13),
        }
        .into(),
        trailing_whitespace: None,
        mustache_close: Some(MustacheCloseToken {
            span: new_span(13, 14),
        }),
        span: new_span(0, 14),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert!(error_reporter.is_empty())
}

#[test]
fn mustache_debug_tag_sequence() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{@debug hello, 123}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: MustacheOpenToken {
            span: new_span(0, 1),
        },
        leading_whitespace: None,
        mustache_item: DebugTag {
            debug_tag: DebugTagToken {
                span: new_span(1, 7),
            },
            whitespace: Some(WhitespaceToken {
                span: new_span(7, 8),
            }),
            identifiers: vec![
                Box::new(Expr::Ident(Ident::new("hello".into(), new_span(8, 13)))),
                Box::new(Expr::Lit(Lit::Num(Number {
                    span: new_span(15, 18),
                    value: 123.0,
                    raw: Some("123".into()),
                }))),
            ],
            span: new_span(1, 18),
        }
        .into(),
        trailing_whitespace: None,
        mustache_close: Some(MustacheCloseToken {
            span: new_span(18, 19),
        }),
        span: new_span(0, 19),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert_eq!(
        error_reporter.parse_errors(),
        &[ParseError::new(
            ParseErrorKind::InvalidDebugArgs,
            new_span(15, 18)
        )]
    );
}

#[test]
fn mustache_debug_all_tag() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{@debug}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: MustacheOpenToken {
            span: new_span(0, 1),
        },
        leading_whitespace: None,
        mustache_item: DebugTag {
            debug_tag: DebugTagToken {
                span: new_span(1, 7),
            },
            whitespace: None,
            identifiers: vec![],
            span: new_span(1, 7),
        }
        .into(),
        trailing_whitespace: None,
        mustache_close: Some(MustacheCloseToken {
            span: new_span(7, 8),
        }),
        span: new_span(0, 8),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert!(error_reporter.is_empty())
}

#[test]
fn mustache_raw_tag() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{@html hello}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: MustacheOpenToken {
            span: new_span(0, 1),
        },
        leading_whitespace: None,
        mustache_item: RawMustacheTag {
            html_tag: HtmlTagToken {
                span: new_span(1, 6),
            },
            whitespace: WhitespaceToken {
                span: new_span(6, 7),
            },
            expression: Box::new(Expr::Ident(Ident::new("hello".into(), new_span(7, 12)))),
            span: new_span(1, 12),
        }
        .into(),
        trailing_whitespace: None,
        mustache_close: Some(MustacheCloseToken {
            span: new_span(12, 13),
        }),
        span: new_span(0, 13),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert!(error_reporter.is_empty())
}

#[test]
fn invalid_block_open() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{#foo}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: MustacheOpenToken {
            span: new_span(0, 1),
        },
        leading_whitespace: None,
        mustache_item: InvalidSyntax {
            text: "#foo".into(),
            span: new_span(1, 5),
        }
        .into(),
        trailing_whitespace: None,
        mustache_close: Some(MustacheCloseToken {
            span: new_span(5, 6),
        }),
        span: new_span(0, 6),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert_eq!(
        error_reporter.parse_errors(),
        &[ParseError::new(
            ParseErrorKind::UnexpectedBlockType,
            new_span(1, 5)
        )]
    );
}

#[test]
fn key_block_open() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{#key hello}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: MustacheOpenToken {
            span: new_span(0, 1),
        },
        leading_whitespace: None,
        mustache_item: KeyBlockOpen {
            key_open: KeyOpenToken {
                span: new_span(1, 5),
            },
            whitespace: WhitespaceToken {
                span: new_span(5, 6),
            },
            expression: Box::new(Expr::Ident(Ident::new("hello".into(), new_span(6, 11)))),
            span: new_span(1, 11),
        }
        .into(),
        trailing_whitespace: None,
        mustache_close: Some(MustacheCloseToken {
            span: new_span(11, 12),
        }),
        span: new_span(0, 12),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert!(error_reporter.is_empty())
}

#[test]
fn if_block_open() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{#if hello}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: MustacheOpenToken {
            span: new_span(0, 1),
        },
        leading_whitespace: None,
        mustache_item: IfBlockOpen {
            if_open: IfOpenToken {
                span: new_span(1, 4),
            },
            whitespace: WhitespaceToken {
                span: new_span(4, 5),
            },
            expression: Box::new(Expr::Ident(Ident::new("hello".into(), new_span(5, 10)))),
            span: new_span(1, 10),
        }
        .into(),
        trailing_whitespace: None,
        mustache_close: Some(MustacheCloseToken {
            span: new_span(10, 11),
        }),
        span: new_span(0, 11),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert!(error_reporter.is_empty())
}

#[test]
fn each_block_open() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{#each items as item}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: new_span(0, 1).into(),
        leading_whitespace: None,
        mustache_item: EachBlockOpen {
            each_open: new_span(1, 6).into(),
            whitespace: new_span(6, 7).into(),
            expression: Box::new(Expr::Ident(Ident::new("items".into(), new_span(7, 12)))),
            as_: EachAs {
                leading_ws: new_span(12, 13).into(),
                as_: new_span(13, 15).into(),
                trailing_ws: new_span(15, 16).into(),
                span: new_span(12, 16),
            },
            context: Identifier {
                name: "item".into(),
                span: new_span(16, 20),
            }
            .into(),
            index: None,
            key: None,
            span: new_span(1, 20),
        }
        .into(),
        trailing_whitespace: None,
        mustache_close: Some(new_span(20, 21).into()),
        span: new_span(0, 21),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert!(error_reporter.is_empty())
}
