mod tokens;

use self::tokens::TOKEN_NAMES;
use crate::tokens::TOKEN_TYPES;
use convert_case::{Case, Casing};
use pluralizer::pluralize;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::{borrow::Cow, path::PathBuf};
use syn::Type;
use ungrammar::{Grammar, Node, Rule, Token};
use xshell::{cmd, Shell};

fn main() -> anyhow::Result<()> {
    let grammar: Grammar = include_str!("../svelte.ungram").parse()?;

    let tokens = grammar.tokens().filter_map(|token_ref| {
        let token_data = &grammar[token_ref];
        let (_, name) = TOKEN_NAMES
            .iter()
            .find(|(name, _)| *name == token_data.name)?;
        let name_ident = format_ident!("{name}Token");
        Some(quote! {
            #[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
            #[ast_serde(#name)]
            pub struct #name_ident {
                pub span: Span,
            }
        })
    });
    let node_structs = grammar.iter().map(|node_ref| {
        let node_data = &grammar[node_ref];
        let name = &node_data.name;
        let name_ident = format_ident!("{name}");
        let mut fields = vec![];
        match &node_data.rule {
            Rule::Seq(rules) => {
                for rule in rules {
                    fields.push(get_simple_field(&grammar, rule, FieldMeta::default()))
                }
            }
            Rule::Alt(rules) => {
                let variants = rules.iter().map(|rule| match rule {
                    Rule::Labeled { .. } => todo!(),
                    Rule::Node(n) => {
                        let node_data = &grammar[*n];
                        let ident = format_ident!("{}", node_data.name);
                        quote! { #ident(#ident) }
                    }
                    Rule::Token(_) => todo!(),
                    Rule::Seq(_) => todo!(),
                    Rule::Alt(_) => todo!(),
                    Rule::Opt(_) => todo!(),
                    Rule::Rep(_) => todo!(),
                });

                return quote! {
                    #[derive(Debug, Spanned, Serialize, Deserialize, EqIgnoreSpan, PartialEq)]
                    #[serde(untagged)]
                    pub enum #name_ident {
                        #(#variants,)*
                    }
                };
            }
            rule => fields.push(get_simple_field(&grammar, rule, FieldMeta::default())),
        }
        quote! {
            #[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
            #[ast_serde(#name)]
            pub struct #name_ident {
                #(#fields,)*
                pub span: Span,
            }
        }
    });
    let node_variants = grammar.iter().map(|node_ref| {
        let node_data = &grammar[node_ref];
        let name = &node_data.name;
        let ident = format_ident!("{name}");
        quote! {
            #ident(#ident)
        }
    });

    std::fs::write(
        project_root().join("src/generated/tokens.rs"),
        reformat(
            quote! {
                use swc_common::{ast_serde, Span, Spanned, EqIgnoreSpan};

                #(#tokens)*
            }
            .to_string(),
        ),
    )?;
    std::fs::write(
        project_root().join("src/generated/nodes.rs"),
        reformat(
            quote! {
                use swc_common::{ast_serde, Span, Spanned, EqIgnoreSpan};
                use serde::{Deserialize, Serialize};
                use derive_more::From;
                use super::tokens::*;

                #[derive(Debug, From, Spanned, Serialize, Deserialize, EqIgnoreSpan, PartialEq)]
                #[serde(untagged)]
                pub enum Node {
                    #(#node_variants,)*
                }

                #(#node_structs)*
            }
            .to_string(),
        ),
    )?;
    std::fs::write(
        project_root().join("src/generated.rs"),
        reformat(
            quote! {
                pub mod nodes;
                pub mod tokens;
            }
            .to_string(),
        ),
    )?;

    Ok(())
}

fn ensure_rustfmt(sh: &Shell) {
    let version = cmd!(sh, "rustup run stable rustfmt --version")
        .read()
        .unwrap_or_default();
    if !version.contains("stable") {
        panic!(
            "Failed to run rustfmt from toolchain 'stable'. \
                 Please run `rustup component add rustfmt --toolchain stable` to install it.",
        );
    }
}
fn reformat(text: String) -> String {
    let sh = Shell::new().unwrap();
    ensure_rustfmt(&sh);
    let mut stdout = cmd!(sh, "rustup run stable rustfmt --config fn_single_line=true")
        .stdin(text)
        .read()
        .unwrap();
    if !stdout.ends_with('\n') {
        stdout.push('\n');
    }
    stdout
}
fn project_root() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    let res = PathBuf::from(dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned();
    assert!(res.join("Cargo.lock").exists());
    res
}

fn get_token_field(grammar: &Grammar, t: &Token, meta: FieldMeta<'_>) -> TokenStream {
    let token_data = &grammar[*t];
    let matched_type = TOKEN_TYPES
        .iter()
        .find(|(token_name, _)| *token_name == token_data.name);
    let matched_name = TOKEN_NAMES
        .iter()
        .find(|(token_name, _)| *token_name == token_data.name);

    match (matched_type, matched_name) {
        (Some((token_name, token_type)), _) => {
            let token_type: Type = meta.parse_type(token_type);
            let field_ident = meta.field_ident(token_name);
            quote! {
                pub #field_ident: #token_type
            }
        }
        (_, Some((_, token_type_name))) => {
            let token_name = token_type_name.to_case(Case::Snake);
            let field_ident = meta.field_ident(&token_name);
            let token_type: Type = meta.parse_type(&format!("{token_type_name}Token"));
            quote! {
                pub #field_ident: #token_type
            }
        }
        _ => panic!("unable to match token to type or name: {}", token_data.name),
    }
}

fn get_node_field(grammar: &Grammar, n: &Node, meta: FieldMeta<'_>) -> TokenStream {
    let node_data = &grammar[*n];
    let node_name = node_data.name.to_case(Case::Snake);
    let field_ident = meta.field_ident(&node_name);
    let field_type: Type = meta.parse_type(&node_data.name);
    quote! {
        pub #field_ident: #field_type
    }
}

fn get_simple_field(grammar: &Grammar, rule: &Rule, meta: FieldMeta<'_>) -> TokenStream {
    match rule {
        Rule::Labeled { label, rule } => get_simple_field(grammar, rule, meta.with_label(label)),
        Rule::Node(n) => get_node_field(grammar, n, meta),
        Rule::Token(t) => get_token_field(grammar, t, meta),
        Rule::Seq(_) => todo!(),
        Rule::Alt(_) => todo!(),
        Rule::Opt(rule) => get_simple_field(grammar, rule, meta.with_flag(FieldFlag::Optional)),
        Rule::Rep(rule) => get_simple_field(grammar, rule, meta.with_flag(FieldFlag::Repeated)),
    }
}

enum FieldFlag {
    Optional,
    Repeated,
}
#[derive(Default)]
struct FieldMeta<'a> {
    label: Option<&'a str>,
    flag: Option<FieldFlag>,
}

impl<'a> FieldMeta<'a> {
    fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    fn with_flag(mut self, flag: FieldFlag) -> Self {
        self.flag = Some(flag);
        self
    }

    fn field_ident(&self, fallback_name: &str) -> Ident {
        let mut name: Cow<str> = match self.label {
            Some(label) if label.ends_with('_') => format!("{label}{fallback_name}").into(),
            Some(label) => label.into(),
            None => fallback_name.into(),
        };

        if matches!(self.flag, Some(FieldFlag::Repeated)) {
            name = pluralize(&name, 2, false).into();
        }

        format_ident!("{name}")
    }

    fn parse_type(&self, type_str: &str) -> Type {
        match self.flag {
            Some(FieldFlag::Optional) => syn::parse_str(&format!("Option<{type_str}>")).unwrap(),
            Some(FieldFlag::Repeated) => syn::parse_str(&format!("Vec<{type_str}>")).unwrap(),
            None => syn::parse_str(type_str).unwrap(),
        }
    }
}
