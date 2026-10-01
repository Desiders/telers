#![allow(clippy::missing_errors_doc)]

use crate::generator::{helpers::camel_to_snake, serialization::configure_serialization};
use std::{fmt::Display, fs, path::Path};

pub fn write_tokens_to_file(
    tokens: impl Display,
    dir: &Path,
    filename: &str,
) -> anyhow::Result<()> {
    let mut syntax_tree = syn::parse_file(&tokens.to_string())?;
    configure_serialization(&mut syntax_tree)?;
    let formatted = prettyplease::unparse(&syntax_tree);
    fs::create_dir_all(dir)?;
    fs::write(dir.join(filename), formatted)?;
    Ok(())
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
