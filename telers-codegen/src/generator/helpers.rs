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

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();
        let is_top_level = !line.starts_with(' ') && !line.starts_with('\t');
        let prev_is_not_blank = i > 0 && !lines[i - 1].trim().is_empty();
        let prev_is_not_doc = i > 0 && !lines[i - 1].trim_start().starts_with("///");

        if is_top_level && trimmed.starts_with("///") && prev_is_not_blank && prev_is_not_doc {
            result.push('\n');
        }

        result.push_str(line);
        result.push('\n');
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
