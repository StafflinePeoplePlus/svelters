use pretty_assertions::assert_eq;
use svelters::{
    ecma::{AssignPatProp, Expr, Ident, Lit, MemberExpr, Number, ObjectPat, Pat},
    error::{CollectingErrorReporter, ParseError, ParseErrorKind},
    parser::{new_span, Parser},
    syntax_nodes::{
        BlockClose, Comment, CommentText, ConstTag, DebugTag, EachAs, EachBlockOpen, EachIndex,
        EachKey, IfBlockOpen, InvalidSyntax, KeyBlockOpen, Mustache, MustacheItem, RawMustacheTag,
        Text,
    },
    tokens::{
        CommentEndToken, CommentStartToken, ConstTagToken, DebugTagToken, HtmlTagToken,
        IfOpenToken, KeyOpenToken, MustacheCloseToken, MustacheOpenToken, WhitespaceToken,
    },
};

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
        mustache_item: MustacheItem::BlockOpen(
            InvalidSyntax {
                text: "#foo".into(),
                span: new_span(1, 5),
            }
            .into(),
        ),
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
        mustache_item: MustacheItem::BlockOpen(
            KeyBlockOpen {
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
        ),
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
        mustache_item: MustacheItem::BlockOpen(
            IfBlockOpen {
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
        ),
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
        mustache_item: MustacheItem::BlockOpen(
            EachBlockOpen {
                each_open: new_span(1, 6).into(),
                whitespace: new_span(6, 7).into(),
                expression: Box::new(Expr::Ident(Ident::new("items".into(), new_span(7, 12)))),
                as_: EachAs {
                    leading_ws: new_span(12, 13).into(),
                    as_: new_span(13, 15).into(),
                    trailing_ws: new_span(15, 16).into(),
                    span: new_span(12, 16),
                },
                context: Pat::Ident(Ident::new("item".into(), new_span(16, 20)).into()),
                index: None,
                key: None,
                span: new_span(1, 20),
            }
            .into(),
        ),
        trailing_whitespace: None,
        mustache_close: Some(new_span(20, 21).into()),
        span: new_span(0, 21),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert!(error_reporter.is_empty())
}

#[test]
fn each_block_open_index() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{#each items as item, $i}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: new_span(0, 1).into(),
        leading_whitespace: None,
        mustache_item: MustacheItem::BlockOpen(
            EachBlockOpen {
                each_open: new_span(1, 6).into(),
                whitespace: new_span(6, 7).into(),
                expression: Box::new(Expr::Ident(Ident::new("items".into(), new_span(7, 12)))),
                as_: EachAs {
                    leading_ws: new_span(12, 13).into(),
                    as_: new_span(13, 15).into(),
                    trailing_ws: new_span(15, 16).into(),
                    span: new_span(12, 16),
                },
                context: Pat::Ident(Ident::new("item".into(), new_span(16, 20)).into()),
                index: Some(EachIndex {
                    trailing_ws: None,
                    comma: new_span(20, 21).into(),
                    whitespace: Some(new_span(21, 22).into()),
                    identifier: Ident::new("$i".into(), new_span(22, 24)),
                    span: new_span(20, 24),
                }),
                key: None,
                span: new_span(1, 24),
            }
            .into(),
        ),
        trailing_whitespace: None,
        mustache_close: Some(new_span(24, 25).into()),
        span: new_span(0, 25),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert!(error_reporter.is_empty())
}

#[test]
fn each_block_open_keyed() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{#each items as item (item.id)}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: new_span(0, 1).into(),
        leading_whitespace: None,
        mustache_item: MustacheItem::BlockOpen(
            EachBlockOpen {
                each_open: new_span(1, 6).into(),
                whitespace: new_span(6, 7).into(),
                expression: Box::new(Expr::Ident(Ident::new("items".into(), new_span(7, 12)))),
                as_: EachAs {
                    leading_ws: new_span(12, 13).into(),
                    as_: new_span(13, 15).into(),
                    trailing_ws: new_span(15, 16).into(),
                    span: new_span(12, 16),
                },
                context: Pat::Ident(Ident::new("item".into(), new_span(16, 20)).into()),
                index: None,
                key: Some(EachKey {
                    whitespace: Some(new_span(20, 21).into()),
                    paren_open: new_span(21, 22).into(),
                    leading_ws: None,
                    expression: Box::new(Expr::Member(MemberExpr {
                        span: new_span(22, 29),
                        obj: Box::new(Ident::new("item".into(), new_span(22, 26)).into()),
                        prop: Ident::new("id".into(), new_span(27, 29)).into(),
                    })),
                    trailing_ws: None,
                    paren_close: new_span(29, 30).into(),
                    span: new_span(20, 30),
                }),
                span: new_span(1, 30),
            }
            .into(),
        ),
        trailing_whitespace: None,
        mustache_close: Some(new_span(30, 31).into()),
        span: new_span(0, 31),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert!(error_reporter.is_empty())
}

#[test]
fn each_block_open_index_keyed() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{#each items as item, i (i)}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: new_span(0, 1).into(),
        leading_whitespace: None,
        mustache_item: MustacheItem::BlockOpen(
            EachBlockOpen {
                each_open: new_span(1, 6).into(),
                whitespace: new_span(6, 7).into(),
                expression: Box::new(Expr::Ident(Ident::new("items".into(), new_span(7, 12)))),
                as_: EachAs {
                    leading_ws: new_span(12, 13).into(),
                    as_: new_span(13, 15).into(),
                    trailing_ws: new_span(15, 16).into(),
                    span: new_span(12, 16),
                },
                context: Pat::Ident(Ident::new("item".into(), new_span(16, 20)).into()),
                index: Some(EachIndex {
                    trailing_ws: None,
                    comma: new_span(20, 21).into(),
                    whitespace: Some(new_span(21, 22).into()),
                    identifier: Ident::new("i".into(), new_span(22, 23)),
                    span: new_span(20, 23),
                }),
                key: Some(EachKey {
                    whitespace: Some(new_span(23, 24).into()),
                    paren_open: new_span(24, 25).into(),
                    leading_ws: None,
                    expression: Box::new(Ident::new("i".into(), new_span(25, 26)).into()),
                    trailing_ws: None,
                    paren_close: new_span(26, 27).into(),
                    span: new_span(23, 27),
                }),
                span: new_span(1, 27),
            }
            .into(),
        ),
        trailing_whitespace: None,
        mustache_close: Some(new_span(27, 28).into()),
        span: new_span(0, 28),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert!(error_reporter.is_empty())
}

#[test]
fn each_block_open_invalid_index() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{#each items as item, 123 (i)}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: new_span(0, 1).into(),
        leading_whitespace: None,
        mustache_item: MustacheItem::BlockOpen(
            EachBlockOpen {
                each_open: new_span(1, 6).into(),
                whitespace: new_span(6, 7).into(),
                expression: Box::new(Expr::Ident(Ident::new("items".into(), new_span(7, 12)))),
                as_: EachAs {
                    leading_ws: new_span(12, 13).into(),
                    as_: new_span(13, 15).into(),
                    trailing_ws: new_span(15, 16).into(),
                    span: new_span(12, 16),
                },
                context: Pat::Ident(Ident::new("item".into(), new_span(16, 20)).into()),
                index: None,
                key: Some(EachKey {
                    whitespace: Some(new_span(25, 26).into()),
                    paren_open: new_span(26, 27).into(),
                    leading_ws: None,
                    expression: Box::new(Ident::new("i".into(), new_span(27, 28)).into()),
                    trailing_ws: None,
                    paren_close: new_span(28, 29).into(),
                    span: new_span(25, 29),
                }),
                span: new_span(1, 29),
            }
            .into(),
        ),
        trailing_whitespace: None,
        mustache_close: Some(new_span(29, 30).into()),
        span: new_span(0, 30),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert_eq!(
        error_reporter.parse_errors(),
        &[ParseError::new(
            ParseErrorKind::ExpectedEachIndex,
            new_span(22, 25)
        )]
    );
}

#[test]
fn each_block_destructure() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{#each items as { id }}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: new_span(0, 1).into(),
        leading_whitespace: None,
        mustache_item: MustacheItem::BlockOpen(
            EachBlockOpen {
                each_open: new_span(1, 6).into(),
                whitespace: new_span(6, 7).into(),
                expression: Box::new(Ident::new("items".into(), new_span(7, 12)).into()),
                as_: EachAs {
                    leading_ws: new_span(12, 13).into(),
                    as_: new_span(13, 15).into(),
                    trailing_ws: new_span(15, 16).into(),
                    span: new_span(12, 16),
                },
                context: ObjectPat {
                    span: new_span(16, 22),
                    props: vec![AssignPatProp {
                        span: new_span(18, 20),
                        key: Ident::new("id".into(), new_span(18, 20)),
                        value: None,
                    }
                    .into()],
                    optional: false,
                    type_ann: None,
                }
                .into(),
                index: None,
                key: None,
                span: new_span(1, 22),
            }
            .into(),
        ),
        trailing_whitespace: None,
        mustache_close: Some(new_span(22, 23).into()),
        span: new_span(0, 23),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert!(error_reporter.is_empty())
}

#[test]
fn key_close() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{/key}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: new_span(0, 1).into(),
        leading_whitespace: None,
        mustache_item: BlockClose::KeyClose(new_span(1, 5).into()).into(),
        trailing_whitespace: None,
        mustache_close: Some(new_span(5, 6).into()),
        span: new_span(0, 6),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert!(error_reporter.is_empty())
}

#[test]
fn if_close() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{/if}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: new_span(0, 1).into(),
        leading_whitespace: None,
        mustache_item: BlockClose::IfClose(new_span(1, 4).into()).into(),
        trailing_whitespace: None,
        mustache_close: Some(new_span(4, 5).into()),
        span: new_span(0, 5),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert!(error_reporter.is_empty())
}

#[test]
fn await_close() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{/await}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: new_span(0, 1).into(),
        leading_whitespace: None,
        mustache_item: BlockClose::AwaitClose(new_span(1, 7).into()).into(),
        trailing_whitespace: None,
        mustache_close: Some(new_span(7, 8).into()),
        span: new_span(0, 8),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert!(error_reporter.is_empty())
}

#[test]
fn each_close() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{/each}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: new_span(0, 1).into(),
        leading_whitespace: None,
        mustache_item: BlockClose::EachClose(new_span(1, 6).into()).into(),
        trailing_whitespace: None,
        mustache_close: Some(new_span(6, 7).into()),
        span: new_span(0, 7),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert!(error_reporter.is_empty())
}

#[test]
fn unknown_close() {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new("{/keys}", &mut error_reporter).parse();
    let expected_node = Mustache {
        mustache_open: new_span(0, 1).into(),
        leading_whitespace: None,
        mustache_item: BlockClose::Unknown(InvalidSyntax {
            text: "/keys".into(),
            span: new_span(1, 6),
        })
        .into(),
        trailing_whitespace: None,
        mustache_close: Some(new_span(6, 7).into()),
        span: new_span(0, 7),
    };

    assert_eq!(nodes, vec![expected_node.into()]);
    assert_eq!(
        error_reporter.parse_errors(),
        &[ParseError::new(
            ParseErrorKind::UnknownBlockClose,
            new_span(2, 6)
        )]
    );
}
