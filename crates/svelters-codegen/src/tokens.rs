pub const TOKEN_NAMES: &[(&str, &str)] = &[
    ("whitespace", "Whitespace"),
    ("{", "MustacheOpen"),
    ("}", "MustacheClose"),
    ("<!--", "CommentStart"),
    ("-->", "CommentEnd"),
    (",", "Comma"),
    ("(", "ParenOpen"),
    (")", "ParenClose"),
    ("@const", "ConstTag"),
    ("@debug", "DebugTag"),
    ("@html", "HtmlTag"),
    ("#key", "KeyOpen"),
    ("#if", "IfOpen"),
    ("#each", "EachOpen"),
    ("/key", "KeyClose"),
    ("/if", "IfClose"),
    ("/each", "EachClose"),
    ("/await", "AwaitClose"),
    ("as", "As"),
];
pub const TOKEN_TYPES: &[(&str, &str)] = &[
    ("expression", "Box<swc_ecma_ast::Expr>"),
    ("pattern", "swc_ecma_ast::Pat"),
    ("identifier", "swc_ecma_ast::Ident"),
    ("text", "String"),
];
