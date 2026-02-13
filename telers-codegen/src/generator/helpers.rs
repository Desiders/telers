use pluralizer::pluralize;
use quote::format_ident;
use std::fmt::Display;

pub fn format_tokens(tokens: impl Display) -> syn::Result<String> {
    let syntax_tree = syn::parse_file(&tokens.to_string())?;
    let formatted = prettyplease::unparse(&syntax_tree);
    Ok(formatted)
}

pub fn sanitize_field_name(name: &str) -> proc_macro2::Ident {
    match name {
        "type" => format_ident!("r#type"),
        "self" => format_ident!("r#self"),
        "ref" => format_ident!("r#ref"),
        "move" => format_ident!("r#move"),
        "use" => format_ident!("r#use"),
        "mod" => format_ident!("r#mod"),
        "impl" => format_ident!("r#impl"),
        "trait" => format_ident!("r#trait"),
        "where" => format_ident!("r#where"),
        _ => format_ident!("{name}"),
    }
}

fn sanitize_description(description: &str) -> &str {
    description
        .strip_prefix("Optional. ")
        .unwrap_or(description)
}

pub fn format_description(description: &[String], href: &str) -> Vec<String> {
    description
        .iter()
        .map(|line| format!(" {}", sanitize_description(line)))
        .chain([format!(" # Documentation"), format!(" <{}>", href)])
        .collect()
}

pub fn format_attr_description(description: &str) -> String {
    format!(" {}", sanitize_description(description))
}

pub fn get_singular_and_plural_forms(name: &str) -> (String, String) {
    (pluralize(name, 1, false), pluralize(name, 2, false))
}
