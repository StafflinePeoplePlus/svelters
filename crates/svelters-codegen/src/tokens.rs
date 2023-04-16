pub const TOKEN_NAMES: &[(&str, &str)] = &[
    ("whitespace", "Whitespace"),
    ("{", "MustacheOpen"),
    ("}", "MustacheClose"),
    ("<!--", "CommentStart"),
    ("-->", "CommentEnd"),
];
pub const TOKEN_TYPES: &[(&str, &str)] = &[
    ("js_expression", "Box<swc_ecma_ast::Expr>"),
    ("text", "String"),
];
