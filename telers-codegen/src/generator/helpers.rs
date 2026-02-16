use pluralizer::pluralize;
use quote::format_ident;
use std::fmt::Display;

pub fn format_tokens(tokens: impl Display) -> syn::Result<String> {
    let syntax_tree = syn::parse_file(&tokens.to_string())?;
    Ok(prettyplease::unparse(&syntax_tree))
}

const RESERVED_KEYWORDS: &[&str] = &[
    "type", "self", "ref", "move", "use", "mod", "impl", "trait", "where",
];

pub fn sanitize_field_name(name: &str) -> proc_macro2::Ident {
    if RESERVED_KEYWORDS.contains(&name) {
        format_ident!("r#{name}")
    } else {
        format_ident!("{name}")
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
        .chain([" # Documentation".to_string(), format!(" <{href}>")])
        .collect()
}

pub fn format_attr_description(description: &str) -> String {
    format!(" {}", sanitize_description(description))
}

pub fn get_singular_and_plural_forms(name: &str) -> (String, String) {
    (pluralize(name, 1, false), pluralize(name, 2, false))
}

pub fn camel_to_snake(input: &str) -> String {
    let mut result = String::with_capacity(input.len() + 3);
    let chars: Vec<char> = input.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            let prev = i.checked_sub(1).map(|j| chars[j]);
            let next = chars.get(i + 1);
            if prev.is_some_and(|p| {
                p.is_lowercase()
                    || p.is_ascii_digit()
                    || (p.is_uppercase() && next.is_some_and(|n| n.is_lowercase()))
            }) {
                result.push('_');
            }
            result.extend(c.to_lowercase());
        } else {
            result.push(c);
        }
    }

    result
}

pub fn snake_to_upper_camel(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut capitalize_next = true;

    for c in input.chars() {
        if c == '_' {
            capitalize_next = true;
        } else {
            if capitalize_next {
                result.extend(c.to_uppercase());
            } else {
                result.push(c);
            }
            capitalize_next = false;
        }
    }

    result
}
