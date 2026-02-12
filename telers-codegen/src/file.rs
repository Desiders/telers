use crate::generator::helpers::format_tokens;

use std::{fmt::Display, fs, path::Path};

pub fn write_tokens_to_file(
    tokens: impl Display,
    dir: &Path,
    filename: &str,
) -> anyhow::Result<()> {
    let formatted = format_tokens(&tokens)?;
    fs::write(dir.join(filename), formatted)?;
    Ok(())
}

pub fn camel_to_rs_filename(input: &str) -> String {
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
                    || (prev.is_uppercase() && next.map_or(false, |n| n.is_lowercase()))
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

    result.push_str(".rs");
    result
}
