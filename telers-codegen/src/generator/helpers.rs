use quote::format_ident;
use std::fmt::Display;

pub fn format_tokens(tokens: impl Display) -> syn::Result<String> {
    let syntax_tree = syn::parse_file(&tokens.to_string())?;
    let formatted = prettyplease::unparse(&syntax_tree);
    Ok(add_blank_lines_between_items(&formatted))
}

fn add_blank_lines_between_items(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut result = String::new();
    let mut i = 0;
    let mut in_use_block = false;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim_start();

        let is_use = trimmed.starts_with("use ");
        let is_top_level = !line.starts_with(' ') && !line.starts_with('\t');
        let is_attribute = trimmed.starts_with("#[");
        let is_doc_comment = trimmed.starts_with("///");
        let is_item_keyword = (trimmed.starts_with("pub")
            || trimmed.starts_with("impl")
            || trimmed.starts_with("const")
            || trimmed.starts_with("fn")
            || trimmed.starts_with("struct")
            || trimmed.starts_with("enum")
            || trimmed.starts_with("type")
            || trimmed.starts_with("mod"))
            && !trimmed.starts_with("pub use");

        if is_use {
            if !in_use_block && i > 0 && !result.is_empty() {
                let prev_line = lines[i - 1];
                if !prev_line.trim_start().starts_with("use ") && !result.ends_with("\n\n") {
                    result.push('\n');
                }
            }
            in_use_block = true;
        } else {
            if in_use_block {
                in_use_block = false;
                if !result.ends_with("\n\n") {
                    result.push('\n');
                }
            }
        }

        if i > 0 && !result.is_empty() && !is_use && !in_use_block {
            let prev_line = lines[i - 1];
            let prev_trimmed = prev_line.trim_start();

            let should_not_add_blank = prev_trimmed.starts_with("///")
                || prev_trimmed.starts_with("#[")
                || (is_doc_comment && !prev_trimmed.starts_with("use "))
                || is_attribute;

            if is_top_level && is_item_keyword && !should_not_add_blank {
                if !result.ends_with("\n\n") {
                    result.push('\n');
                }
            }
        }

        result.push_str(line);
        result.push('\n');
        i += 1;
    }

    result
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

pub fn format_description(description: Vec<impl Display>, href: impl Display) -> Vec<String> {
    description
        .iter()
        .map(|line| format!(" {line}"))
        .chain([format!(" # Documentation"), format!(" <{}>", href)])
        .collect()
}

pub fn format_attr_description(description: impl Display) -> String {
    format!(" {description}")
}
