pub mod ast;
pub mod lexer;
pub mod parser;
pub mod syntax;

// use crate::{ast::Fragment, lexer::lex, parser::Parser};
// use rowan::ast::AstNode;
// fn main() {
//     let source = include_str!("../source.svelte");
//     let tokens = lex(source);
//     println!("{tokens:#?}");

//     let syntax = Parser::new(source, tokens).parse();
//     println!("{syntax:#?}");

//     let ast = Fragment::cast(syntax);
//     println!("{ast:#?}");
// }
