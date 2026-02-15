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
        .chain([" # Documentation".to_string(), format!(" <{}>", href)])
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

    for i in 0..chars.len() {
        let c = chars[i];

        if c.is_uppercase() {
            if i > 0 {
                let prev = chars[i - 1];
                let next = chars.get(i + 1);

                if prev.is_lowercase()
                    || prev.is_ascii_digit()
                    || (prev.is_uppercase() && next.is_some_and(|n| n.is_lowercase()))
                {
                    result.push('_');
                }
            }

            for lower in c.to_lowercase() {
                result.push(lower);
            }
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
        } else if capitalize_next {
            for upper in c.to_uppercase() {
                result.push(upper);
            }
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}
