use crate::syntax::{
    SyntaxKind::{self, *},
    SyntaxNode,
};
use rowan::GreenNodeBuilder;
use std::iter::Peekable;

pub struct Parser<'a, T>
where
    T: IntoIterator<Item = (SyntaxKind, usize)>,
{
    builder: GreenNodeBuilder<'static>,
    iter: Peekable<T::IntoIter>,
    source: &'a str,
}

impl<'a, T> Parser<'a, T>
where
    T: IntoIterator<Item = (SyntaxKind, usize)>,
{
    pub fn new(source: &'a str, tokens: T) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            iter: tokens.into_iter().peekable(),
            source,
        }
    }

    pub fn parse(mut self) -> SyntaxNode {
        self.parse_fragment(true);

        SyntaxNode::new_root(self.builder.finish())
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        while self
            .iter
            .peek()
            .map(|&(t, _)| t == WHITESPACE)
            .unwrap_or(false)
        {
            self.bump();
        }
        self.iter.peek().map(|&(t, _)| t)
    }

    fn bump(&mut self) {
        if let Some((token, start)) = self.iter.next() {
            let text = match self.iter.peek() {
                Some((_, end)) => &self.source[start..*end],
                None => &self.source[start..],
            };
            self.builder.token(token.into(), text);
        }
    }

    fn error(&mut self) {
        self.builder.start_node(ERROR.into());
        self.bump();
        self.builder.finish_node();
    }

    fn eat(&mut self, token: SyntaxKind) {
        match self.peek() {
            Some(k) if k == token => self.bump(),
            _ => self.error(),
        }
    }

    fn eat_one_of(&mut self, tokens: &[SyntaxKind]) -> SyntaxKind {
        match self.peek() {
            Some(k) if tokens.contains(&k) => {
                self.bump();
                k
            }
            _ => {
                self.error();
                ERROR
            }
        }
    }

    fn parse_fragment(&mut self, error_on_unknown: bool) {
        self.builder.start_node(FRAGMENT.into());
        loop {
            match self.peek() {
                Some(LEFT_ANGLE) => self.parse_element(),
                // Some(MUSTACHE) => todo!(),
                Some(TEXT) => self.parse_text(),
                Some(_) if error_on_unknown => self.error(),
                _ => break,
            }
        }
        self.builder.finish_node();
    }

    fn parse_text(&mut self) {
        self.builder.start_node(TEXT_NODE.into());
        self.eat(TEXT);
        self.builder.finish_node();
    }

    fn parse_element(&mut self) {
        self.builder.start_node(ELEMENT.into());
        if !self.parse_element_open() {
            self.parse_fragment(false);
            self.parse_element_close();
        }
        self.builder.finish_node();
    }

    /// Returns true if the element was self closing
    fn parse_element_open(&mut self) -> bool {
        self.builder.start_node(ELEMENT_OPEN.into());
        self.bump();
        self.eat(TAG_NAME);
        self.parse_attributes();
        let is_self_closing = match self.eat_one_of(&[RIGHT_ANGLE, SLASH_ANGLE]) {
            RIGHT_ANGLE => false,
            SLASH_ANGLE => true,
            _ => true,
        };
        self.builder.finish_node();
        is_self_closing
    }

    fn parse_attributes(&mut self) {
        while let Some(ATTRIBUTE_NAME) = self.peek() {
            self.parse_attribute()
        }
    }

    fn parse_attribute(&mut self) {
        self.builder.start_node(ATTRIBUTE.into());
        self.eat(ATTRIBUTE_NAME);
        self.eat(EQUALS);
        self.eat(ATTRIBUTE_VALUE);
        self.builder.finish_node();
    }

    fn parse_element_close(&mut self) {
        self.builder.start_node(ELEMENT_CLOSE.into());
        self.bump();
        self.eat(TAG_NAME);
        self.eat(RIGHT_ANGLE);
        self.builder.finish_node();
    }
}
