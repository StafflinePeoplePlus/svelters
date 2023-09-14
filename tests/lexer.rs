use pretty_assertions::assert_eq;
use svelters::{lexer::lex, syntax::SyntaxKind::*};

#[test]
fn text() {
    let tokens = lex("Hello, world!");
    assert_eq!(tokens, vec![(TEXT, 0)])
}

#[test]
fn element_open() {
    let tokens = lex("<div>");
    assert_eq!(
        tokens,
        vec![(LEFT_ANGLE, 0), (TAG_NAME, 1), (RIGHT_ANGLE, 4)]
    )
}

#[test]
fn element_open_attributes() {
    let tokens = lex(r#"<div a=a b='b' c="c">"#);
    assert_eq!(
        tokens,
        vec![
            (LEFT_ANGLE, 0),
            (TAG_NAME, 1),
            (WHITESPACE, 4),
            (ATTRIBUTE_NAME, 5),
            (EQUALS, 6),
            (ATTRIBUTE_VALUE, 7),
            (WHITESPACE, 8),
            (ATTRIBUTE_NAME, 9),
            (EQUALS, 10),
            (ATTRIBUTE_VALUE, 11),
            (WHITESPACE, 14),
            (ATTRIBUTE_NAME, 15),
            (EQUALS, 16),
            (ATTRIBUTE_VALUE, 17),
            (RIGHT_ANGLE, 20)
        ]
    )
}

#[test]
fn element_close() {
    let tokens = lex("</div>");
    assert_eq!(
        tokens,
        vec![(ANGLE_SLASH, 0), (TAG_NAME, 2), (RIGHT_ANGLE, 5)]
    )
}

#[test]
fn element_close_whitespace() {
    let tokens = lex("</div  >");
    assert_eq!(
        tokens,
        vec![
            (ANGLE_SLASH, 0),
            (TAG_NAME, 2),
            (WHITESPACE, 5),
            (RIGHT_ANGLE, 7),
        ]
    )
}
