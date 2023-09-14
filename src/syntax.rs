#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[repr(u16)]
pub enum SyntaxKind {
    WHITESPACE,
    LEFT_ANGLE,      // <
    RIGHT_ANGLE,     // <
    ANGLE_SLASH,     // </
    SLASH_ANGLE,     // />
    FORWARD_SLASH,   // /
    LEFT_CURLY,      // {
    EQUALS,          // =
    TAG_NAME,        // HTML tag name
    ATTRIBUTE_NAME,  // HTML attribute name
    ATTRIBUTE_VALUE, // HTML attribute value
    TEXT,

    ELEMENT,
    ELEMENT_OPEN,
    ELEMENT_CLOSE,
    ATTRIBUTE,
    TEXT_NODE,

    ERROR,
    FRAGMENT,
}

impl SyntaxKind {
    /// True if this token is valid to appear before an html tag name.
    pub fn is_pre_tag_name(&self) -> bool {
        self == &SyntaxKind::LEFT_ANGLE || self == &SyntaxKind::ANGLE_SLASH
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SvelteLang {}
impl rowan::Language for SvelteLang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::FRAGMENT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = rowan::SyntaxNode<SvelteLang>;
pub type SyntaxToken = rowan::SyntaxToken<SvelteLang>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
