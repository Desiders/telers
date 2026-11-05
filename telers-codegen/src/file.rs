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

/// Report the path back when the file on disk differs from what the generator would write.
///
/// Compare normalized syntax trees so formatting applied afterwards does not count as drift.
/// Literal contents remain significant. A file that does not exist, or does not parse, differs.
pub fn check_tokens_to_file(
    tokens: impl Display,
    dir: &Path,
    filename: &str,
) -> anyhow::Result<Option<String>> {
    // Canonicalize opaque attribute and macro token streams before comparing trees;
    // their punctuation spacing can differ after the writer's formatting pass.
    let syntax_tree = syn::parse_file(&format_tokens(&tokens)?)?;
    let path = dir.join(filename);
    let matches = fs::read_to_string(&path)
        .ok()
        .and_then(|existing| format_tokens(&existing).ok())
        .and_then(|formatted| syn::parse_file(&formatted).ok())
        .is_some_and(|existing| existing == syntax_tree);

    Ok((!matches).then(|| path.display().to_string()))
}

fn format_tokens(tokens: &impl Display) -> anyhow::Result<String> {
    Ok(prettyplease::unparse(&normalize_tokens(tokens)?))
}

fn normalize_tokens(tokens: &impl Display) -> anyhow::Result<syn::File> {
    let mut syntax_tree = syn::parse_file(&tokens.to_string())?;
    configure_serialization(&mut syntax_tree)?;
    Ok(syntax_tree)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_ignores_blank_lines_between_impl_methods() {
        let dir =
            std::env::temp_dir().join(format!("telers-codegen-spacing-{}", std::process::id()));
        let expected = "struct Example; impl Example { fn first() {} fn second() {} }";
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("example.rs"),
            "struct Example;\n\nimpl Example {\n    fn first() {}\n\n\n    fn second() {}\n}\n",
        )
        .unwrap();

        assert_eq!(
            check_tokens_to_file(expected, &dir, "example.rs").unwrap(),
            None
        );

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn check_preserves_blank_lines_inside_multiline_literals() {
        let dir =
            std::env::temp_dir().join(format!("telers-codegen-literal-{}", std::process::id()));
        let expected = "const TEXT: &str = r#\"first\n\nsecond\"#;";
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("example.rs"),
            "const TEXT: &str = r#\"first\nsecond\"#;",
        )
        .unwrap();

        assert!(
            check_tokens_to_file(expected, &dir, "example.rs")
                .unwrap()
                .is_some()
        );

        fs::remove_dir_all(dir).unwrap();
    }
}
