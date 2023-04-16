pub const TOKEN_NAMES: &[(&str, &str)] = &[
    ("whitespace", "Whitespace"),
    ("{", "MustacheOpen"),
    ("}", "MustacheClose"),
    ("<!--", "CommentStart"),
    ("-->", "CommentEnd"),
    ("@const", "ConstTag"),
];
pub const TOKEN_TYPES: &[(&str, &str)] = &[
    ("expression", "Box<swc_ecma_ast::Expr>"),
    ("text", "String"),
];
