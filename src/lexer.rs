use crate::syntax::SyntaxKind::{self, *};
use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

pub fn lex(text: &str) -> Vec<(SyntaxKind, usize)> {
    let mut lexer = Lexer {
        chars: text.chars().enumerate().peekable(),
        tokens: vec![],
        state: LexerState::default(),
        eof: false,
    };
    while !lexer.eof {
        lexer.tokenize();
    }
    lexer.tokens
}

#[derive(Debug, Default)]
enum LexerState {
    #[default]
    Fragment,
    Tag,
    Mustache,
    Text,
}

struct Lexer<'a> {
    chars: Peekable<Enumerate<Chars<'a>>>,
    tokens: Vec<(SyntaxKind, usize)>,
    state: LexerState,
    eof: bool,
}

impl<'a> Lexer<'a> {
    fn tokenize(&mut self) {
        match self.state {
            LexerState::Fragment => self.tokenize_fragment(),
            LexerState::Tag => self.tokenize_tag(),
            LexerState::Mustache => {
                self.error("Mustache state not implemented yet");
                self.state = LexerState::default();
            }
            LexerState::Text => self.tokenize_text(),
        }
    }

    fn error(&mut self, message: &str) {
        eprintln!("[Lexer Error]: {message}");
        let index = self.tokens.last().map(|token| token.1).unwrap_or_default();
        self.tokens.push((ERROR, index + 1));
    }

    fn match_prev_token<F>(&self, predicate: F) -> bool
    where
        F: FnOnce(SyntaxKind) -> bool,
    {
        self.tokens
            .last()
            .map(|token| predicate(token.0))
            .unwrap_or_default()
    }

    fn tokenize_fragment(&mut self) {
        let Some((_, c)) = self.chars.peek() else {
            self.eof = true;
            return;
        };

        match c {
            '<' => {
                let (start, _) = self.chars.next().unwrap();
                match self.chars.peek() {
                    Some((_, '/')) => {
                        let _ = self.chars.next().unwrap();
                        self.tokens.push((ANGLE_SLASH, start));
                    }
                    _ => {
                        self.tokens.push((LEFT_ANGLE, start));
                    }
                }
                self.state = LexerState::Tag;
            }
            '{' => {
                let (start, _) = self.chars.next().unwrap();
                self.tokens.push((LEFT_CURLY, start));
                self.state = LexerState::Mustache;
            }
            _ => {
                self.state = LexerState::Text;
            }
        }
    }

    fn tokenize_text(&mut self) {
        let Some((start, _)) = self.chars.next() else {
            self.eof = true;
            return;
        };
        while let Some((_, c)) = self.chars.peek() {
            if c == &'<' || c == &'{' {
                break;
            }
            let _ = self.chars.next();
        }

        self.tokens.push((TEXT, start));
        self.state = LexerState::default();
    }

    fn tokenize_tag(&mut self) {
        let Some(&(_, c)) = self.chars.peek() else {
            self.eof = true;
            return;
        };

        match c {
            '=' => {
                let (start, _) = self.chars.next().unwrap();
                self.tokens.push((EQUALS, start));
            }
            '>' => {
                let (start, _) = self.chars.next().unwrap();
                self.tokens.push((RIGHT_ANGLE, start));
                self.state = LexerState::default();
            }
            '/' => {
                let (start, _) = self.chars.next().unwrap();
                let Some((_, '>')) = self.chars.peek() else {
                    self.error("Expected right angle bracket");
                    return;
                };

                let _ = self.chars.next().unwrap();
                self.tokens.push((SLASH_ANGLE, start));
                self.state = LexerState::default();
            }
            c if c.is_ascii_whitespace() => {
                let (start, _) = self.chars.next().unwrap();
                while let Some((_, c)) = self.chars.peek() {
                    if !c.is_ascii_whitespace() {
                        break;
                    }
                    let _ = self.chars.next();
                }
                self.tokens.push((WHITESPACE, start));
            }
            c if c.is_ascii_alphabetic() && self.match_prev_token(|k| k.is_pre_tag_name()) => {
                let (start, _) = self.chars.next().unwrap();
                while let Some((_, c)) = self.chars.peek() {
                    if c.is_ascii_whitespace() || c == &'>' || c == &'/' {
                        break;
                    }
                    let _ = self.chars.next();
                }
                self.tokens.push((TAG_NAME, start));
            }
            c if c.is_ascii_alphabetic() && self.match_prev_token(|k| k == WHITESPACE) => {
                let (start, _) = self.chars.next().unwrap();
                while let Some((_, c)) = self.chars.peek() {
                    if c.is_ascii_whitespace() || c == &'>' || c == &'/' || c == &'=' {
                        break;
                    }
                    let _ = self.chars.next();
                }
                self.tokens.push((ATTRIBUTE_NAME, start));
            }
            c if c.is_attribute_value_start() && self.match_prev_token(|k| k == EQUALS) => {
                // TODO: match mustaches within attribute value?
                let (start, first_char) = self.chars.next().unwrap();
                let is_quoteless = !first_char.is_quote();
                while let Some((_, c)) = self.chars.peek() {
                    if is_quoteless && !c.is_unquoted_attribute_value() {
                        break;
                    }
                    if !is_quoteless && c == &first_char {
                        let _ = self.chars.next();
                        break;
                    }
                    let _ = self.chars.next();
                }
                self.tokens.push((ATTRIBUTE_VALUE, start));
            }
            // TODO: mustache attribute value
            _ => {
                let _ = self.chars.next().unwrap();
                self.error("not implemented");
            }
        }
    }
}

trait SvelteChars {
    fn is_quote(&self) -> bool;
    fn is_unquoted_attribute_value(&self) -> bool;
    fn is_attribute_value_start(&self) -> bool;
}

impl SvelteChars for char {
    fn is_quote(&self) -> bool {
        self == &'"' || self == &'\''
    }
    fn is_unquoted_attribute_value(&self) -> bool {
        !(self.is_ascii_whitespace()
            || self == &'"'
            || self == &'\''
            || self == &'='
            || self == &'<'
            || self == &'>')
    }
    fn is_attribute_value_start(&self) -> bool {
        self.is_unquoted_attribute_value() || self.is_quote()
    }
}
