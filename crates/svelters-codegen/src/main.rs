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
        let serde_name = format!("{name}Token");
        let name_ident = format_ident!("{name}Token");
        Some(quote! {
            #[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
            #[ast_serde(#serde_name)]
            pub struct #name_ident {
                pub span: Span,
            }
            impl From<Span> for #name_ident {
                fn from(span: Span) -> Self {
                    Self { span }
                }
            }
        })
    });
    let node_structs = grammar.iter().map(|node_ref| {
        let node_data = &grammar[node_ref];
        let name = &node_data.name;
        let name_ident = format_ident!("{name}");

        let mut is_enum = false;
        let fields = match &node_data.rule {
            Rule::Alt(rules) => {
                is_enum = true;
                rules
                    .iter()
                    .map(|rule| get_simple_field(&grammar, rule, FieldMeta::default()))
                    .collect()
            }
            Rule::Seq(rules) => rules
                .iter()
                .map(|rule| get_simple_field(&grammar, rule, FieldMeta::default()))
                .collect(),
            rule => {
                vec![get_simple_field(&grammar, rule, FieldMeta::default())]
            }
        };

        if is_enum {
            let num_alts = fields.iter().filter(|f| f.is_alt).count();
            let variants = fields
                .into_iter()
                .map(|f| f.into_enum_variant(num_alts <= 1));
            if num_alts > 1 {
                quote! {
                    #[derive(Debug, Spanned, EqIgnoreSpan, PartialEq, From, Serialize, Deserialize)]
                    #[serde(untagged)]
                    pub enum #name_ident {
                        #(#variants,)*
                    }
                }
            } else {
                quote! {
                    #[derive(Debug, Spanned, EqIgnoreSpan, PartialEq, From)]
                    #[ast_serde]
                    pub enum #name_ident {
                        #(#variants,)*
                    }
                }
            }
        } else {
            let fields = fields.into_iter().map(|f| f.into_struct_field());
            quote! {
                #[derive(Debug, Spanned, EqIgnoreSpan, PartialEq)]
                #[ast_serde(#name)]
                pub struct #name_ident {
                    #(#fields,)*
                    pub span: Span,
                }
            }
        }
    });
    let node_variants = grammar.iter().filter_map(|node_ref| {
        let node_data = &grammar[node_ref];
        if matches!(node_data.rule, Rule::Alt(..)) {
            return None;
        }

        let name = &node_data.name;
        let ident = format_ident!("{name}");
        Some(quote! {
            #[tag(#name)]
            #ident(#ident)
        })
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
        project_root().join("src/generated/syntax_nodes.rs"),
        reformat(
            quote! {
                use swc_common::{ast_serde, Span, Spanned, EqIgnoreSpan};
                use serde::{Deserialize, Serialize};
                use derive_more::From;
                use super::tokens::*;

                #[derive(Debug, From, Spanned, EqIgnoreSpan, PartialEq)]
                #[ast_serde]
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
                pub mod syntax_nodes;
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

fn get_token_field<'a>(grammar: &'a Grammar, t: &'a Token, meta: FieldMeta<'a>) -> Field<'a> {
    let token_data = &grammar[*t];
    let matched_type = TOKEN_TYPES
        .iter()
        .find(|(token_name, _)| *token_name == token_data.name);
    let matched_name = TOKEN_NAMES
        .iter()
        .find(|(token_name, _)| *token_name == token_data.name);

    match (matched_type, matched_name) {
        (Some((token_name, token_type)), _) => Field {
            name: (*token_name).into(),
            type_str: (*token_type).into(),
            meta,
            is_alt: false,
        },
        (_, Some((_, token_type_name))) => {
            let token_name = token_type_name.to_case(Case::Snake);
            Field {
                name: token_name.into(),
                type_str: format!("{token_type_name}Token").into(),
                meta,
                is_alt: false,
            }
        }
        _ => panic!("unable to match token to type or name: {}", token_data.name),
    }
}

fn get_node_field<'a>(grammar: &'a Grammar, n: &'a Node, meta: FieldMeta<'a>) -> Field<'a> {
    let node_data = &grammar[*n];
    let node_name = node_data.name.to_case(Case::Snake);
    Field {
        name: node_name.into(),
        type_str: (&node_data.name).into(),
        meta,
        is_alt: matches!(node_data.rule, Rule::Alt(..)),
    }
}

fn get_simple_field<'a>(grammar: &'a Grammar, rule: &'a Rule, meta: FieldMeta<'a>) -> Field<'a> {
    match rule {
        Rule::Labeled { label, rule } => get_simple_field(grammar, rule, meta.with_label(label)),
        Rule::Node(n) => get_node_field(grammar, n, meta),
        Rule::Token(t) => get_token_field(grammar, t, meta),
        Rule::Opt(rule) => get_simple_field(grammar, rule, meta.with_flag(FieldFlag::Optional)),
        Rule::Rep(rule) => get_simple_field(grammar, rule, meta.with_flag(FieldFlag::Repeated)),
        Rule::Seq(_) => panic!("sequence rule not implemented for simple field"),
        Rule::Alt(_) => panic!("alt rule not implemented for simple field"),
    }
}

struct Field<'a> {
    name: Cow<'a, str>,
    type_str: Cow<'a, str>,
    meta: FieldMeta<'a>,
    is_alt: bool,
}
impl<'a> Field<'a> {
    fn into_struct_field(self) -> TokenStream {
        let field_ident = self.meta.struct_field_ident(&self.name);
        let field_type: Type = self.meta.parse_type(&self.type_str);
        quote! {
            pub #field_ident: #field_type
        }
    }

    fn into_enum_variant(self, include_tag: bool) -> TokenStream {
        let enum_ident = self.meta.enum_variant_ident(&self.name);
        let tag = if self.is_alt {
            "*".to_string()
        } else {
            enum_ident.to_string()
        };
        let field_type: Type = self.meta.parse_type(&self.type_str);

        if include_tag {
            quote! {
                #[tag(#tag)]
                #enum_ident(#field_type)
            }
        } else {
            quote! {
                #enum_ident(#field_type)
            }
        }
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

    fn struct_field_ident(&self, fallback_name: &str) -> Ident {
        let mut name: Cow<str> = match self.label {
            Some(label) => label.into(),
            None => fallback_name.into(),
        };

        if matches!(self.flag, Some(FieldFlag::Repeated)) {
            name = pluralize(&name, 2, false).into();
        }

        format_ident!("{name}")
    }

    fn enum_variant_ident(&self, fallback_name: &str) -> Ident {
        let mut name: Cow<str> = match self.label {
            Some(label) if label.ends_with('_') => format!("{label}{fallback_name}").into(),
            Some(label) => label.into(),
            None => fallback_name.into(),
        };

        if matches!(self.flag, Some(FieldFlag::Repeated)) {
            name = pluralize(&name, 2, false).into();
        }

        format_ident!("{}", name.to_case(Case::Pascal))
    }

    fn parse_type(&self, type_str: &str) -> Type {
        match self.flag {
            Some(FieldFlag::Optional) => syn::parse_str(&format!("Option<{type_str}>")).unwrap(),
            Some(FieldFlag::Repeated) => syn::parse_str(&format!("Vec<{type_str}>")).unwrap(),
            None => syn::parse_str(type_str).unwrap(),
        }
    }
}
