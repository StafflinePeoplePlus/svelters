pub const TOKEN_NAMES: &[(&str, &str)] = &[
    ("whitespace", "Whitespace"),
    ("{", "MustacheOpen"),
    ("}", "MustacheClose"),
    ("<!--", "CommentStart"),
    ("-->", "CommentEnd"),
    ("@const", "ConstTag"),
    ("@debug", "DebugTag"),
    ("@html", "HtmlTag"),
];
pub const TOKEN_TYPES: &[(&str, &str)] = &[
    ("expression", "Box<swc_ecma_ast::Expr>"),
    ("text", "String"),
];
