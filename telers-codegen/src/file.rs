#![allow(clippy::missing_errors_doc)]

use crate::generator::{helpers::camel_to_snake, serialization::configure_serialization};
use std::{fmt::Display, fs, path::Path};

pub fn write_tokens_to_file(
    tokens: impl Display,
    dir: &Path,
    filename: &str,
) -> anyhow::Result<()> {
    let formatted = format_tokens(&tokens)?;
    fs::create_dir_all(dir)?;
    fs::write(dir.join(filename), formatted)?;
    Ok(())
}

/// Render tokens the same way [`write_tokens_to_file`] does and report the path back when the
/// file on disk differs. A file that does not exist counts as differing.
pub fn check_tokens_to_file(
    tokens: impl Display,
    dir: &Path,
    filename: &str,
) -> anyhow::Result<Option<String>> {
    let formatted = format_tokens(&tokens)?;
    let path = dir.join(filename);
    let existing = fs::read_to_string(&path).unwrap_or_default();

    Ok((existing != formatted).then(|| path.display().to_string()))
}

fn format_tokens(tokens: &impl Display) -> anyhow::Result<String> {
    let mut syntax_tree = syn::parse_file(&tokens.to_string())?;
    configure_serialization(&mut syntax_tree)?;

    Ok(prettyplease::unparse(&syntax_tree))
}

#[must_use]
pub fn camel_to_filename(input: &str, ext: Option<&str>) -> String {
    let mut result = camel_to_snake(input);
    if let Some(ext) = ext {
        result.push('.');
        result.push_str(ext);
    }
    result
}
