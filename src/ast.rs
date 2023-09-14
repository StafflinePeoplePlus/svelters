use crate::syntax::{
    SvelteLang,
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken,
};
use rowan::ast::{support, AstChildren, AstNode};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Fragment {
    syntax: SyntaxNode,
}
impl std::fmt::Debug for Fragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Fragment")
            .field(&self.items().collect::<Vec<_>>())
            .finish()
    }
}
impl AstNode for Fragment {
    type Language = SvelteLang;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FRAGMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then_some(Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Fragment {
    pub fn items(&self) -> AstChildren<FragmentItem> {
        support::children(&self.syntax)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum FragmentItem {
    Element(Element),
    // Mustache,
    Text(Text),
}

impl std::fmt::Debug for FragmentItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Element(el) => el.fmt(f),
            Self::Text(text) => text.fmt(f),
        }
    }
}
impl AstNode for FragmentItem {
    type Language = SvelteLang;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ELEMENT || kind == TEXT_NODE
    }
    fn cast(node: SyntaxNode) -> Option<Self> {
        match node.kind() {
            TEXT_NODE => Text::cast(node).map(Self::Text),
            ELEMENT => Element::cast(node).map(Self::Element),
            _ => None,
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            FragmentItem::Element(el) => el.syntax(),
            FragmentItem::Text(text) => text.syntax(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Text {
    syntax: SyntaxNode,
}
impl std::fmt::Debug for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Text")
            .field(
                &self
                    .text()
                    .map(|t| t.text().to_string())
                    .unwrap_or_default(),
            )
            .finish()
    }
}
impl AstNode for Text {
    type Language = SvelteLang;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TEXT_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then_some(Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Text {
    pub fn text(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, TEXT)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Element {
    syntax: SyntaxNode,
}
impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("element_open", &self.element_open())
            .field("fragment", &self.fragment())
            .field("element_close", &self.element_close())
            .finish()
    }
}
impl AstNode for Element {
    type Language = SvelteLang;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then_some(Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Element {
    fn element_open(&self) -> Option<ElementOpen> {
        support::child(&self.syntax)
    }
    fn fragment(&self) -> Option<Fragment> {
        support::child(&self.syntax)
    }
    fn element_close(&self) -> Option<ElementClose> {
        support::child(&self.syntax)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ElementOpen {
    syntax: SyntaxNode,
}
impl std::fmt::Debug for ElementOpen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ElementOpen")
            .field("tag_open_token", &self.tag_open_token())
            .field("tag_name", &self.tag_name().map(|t| t.text().to_string()))
            .field("attributes", &self.attributes().collect::<Vec<_>>())
            .field("tag_close_token", &self.tag_close_token())
            .finish()
    }
}
impl AstNode for ElementOpen {
    type Language = SvelteLang;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ELEMENT_OPEN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then_some(Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ElementOpen {
    fn tag_open_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, LEFT_ANGLE)
    }
    fn tag_name(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, TAG_NAME)
    }
    fn attributes(&self) -> AstChildren<Attribute> {
        support::children(&self.syntax)
    }
    fn tag_close_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, RIGHT_ANGLE).or(support::token(&self.syntax, SLASH_ANGLE))
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ElementClose {
    syntax: SyntaxNode,
}
impl std::fmt::Debug for ElementClose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ElementClose")
            .field("angle_slash_token", &self.angle_slash_token())
            .field("tag_name", &self.tag_name().map(|t| t.text().to_string()))
            .field("right_angle_token", &self.right_angle_token())
            .finish()
    }
}
impl AstNode for ElementClose {
    type Language = SvelteLang;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ELEMENT_CLOSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then_some(Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ElementClose {
    fn angle_slash_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, ANGLE_SLASH)
    }
    fn tag_name(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, TAG_NAME)
    }
    fn right_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, RIGHT_ANGLE)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Attribute {
    syntax: SyntaxNode,
}
impl std::fmt::Debug for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Attribute")
            .field("name", &self.name().map(|t| t.text().to_string()))
            .field("equals_token", &self.equals_token())
            .field("value", &self.value().map(|t| t.text().to_string()))
            .finish()
    }
}
impl AstNode for Attribute {
    type Language = SvelteLang;
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ATTRIBUTE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then_some(Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl Attribute {
    fn name(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, ATTRIBUTE_NAME)
    }
    fn equals_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, EQUALS)
    }
    fn value(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, ATTRIBUTE_VALUE)
    }
}
