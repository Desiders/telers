#![allow(clippy::missing_errors_doc)]

use crate::generator::{helpers::camel_to_snake, serialization::configure_serialization};
use anyhow::Context;
use semver::Version;
use std::{fmt::Display, fs, mem, path::Path};
use toml_edit::{DocumentMut, Item, Value};

pub fn update_manifest_versions(
    dir: &Path,
    api_version: &Version,
    check: bool,
) -> anyhow::Result<Option<String>> {
    let path = dir.join("Cargo.toml");
    let mut manifest = fs::read_to_string(&path)?.parse::<DocumentMut>()?;
    let mut changed = false;
    let mut set_version = |value: &mut Value, version: String| {
        if value.as_str() != Some(version.as_str()) {
            let decor = value.decor().clone();
            *value = Value::from(version);
            *value.decor_mut() = decor;
            changed = true;
        }
    };

    if matches!(
        manifest["package"]["name"].as_str(),
        Some("telers-types" | "telers-methods")
    ) {
        let value = manifest["package"]["version"]
            .as_value_mut()
            .context("expected a string package.version")?;
        let mut version = value
            .as_str()
            .context("expected a string package.version")?
            .parse::<Version>()?;
        version.major = api_version.major;
        version.minor = api_version.minor;
        set_version(value, version.to_string());
    }

    for name in ["telers-types", "telers-methods"] {
        if let Some(value) = manifest
            .get_mut("dependencies")
            .and_then(Item::as_table_like_mut)
            .and_then(|dependencies| dependencies.get_mut(name))
            .and_then(Item::as_table_like_mut)
            .and_then(|dependency| dependency.get_mut("version"))
            .and_then(Item::as_value_mut)
        {
            set_version(value, api_version.major.to_string());
        }
    }

    if !changed {
        return Ok(None);
    }
    if check {
        return Ok(Some(path.display().to_string()));
    }

    fs::write(&path, manifest.to_string())?;
    Ok(None)
}

pub fn write_tokens_to_file(
    tokens: impl Display,
    dir: &Path,
    filename: &str,
) -> anyhow::Result<()> {
    let formatted = format_tokens(&tokens)?;
    let path = dir.join(filename);
    if fs::read_to_string(&path)
        .ok()
        .and_then(|existing| format_tokens(&existing).ok())
        .is_some_and(|existing| existing == formatted)
    {
        return Ok(());
    }
    fs::create_dir_all(dir)?;
    fs::write(path, formatted)?;
    Ok(())
}

/// Report the path back when the file on disk differs from what the generator would write.
///
/// The file on disk is rendered the same way, so formatting applied afterwards does not count as
/// drift. A file that does not exist, or does not parse, counts as differing.
pub fn check_tokens_to_file(
    tokens: impl Display,
    dir: &Path,
    filename: &str,
) -> anyhow::Result<Option<String>> {
    let formatted = format_tokens(&tokens)?;
    let path = dir.join(filename);
    let matches = fs::read_to_string(&path)
        .ok()
        .and_then(|existing| format_tokens(&existing).ok())
        .is_some_and(|existing| existing == formatted);

    Ok((!matches).then(|| path.display().to_string()))
}

fn format_tokens(tokens: &impl Display) -> anyhow::Result<String> {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum ItemGroup {
        Imports,
        Modules,
    }

    let mut syntax_tree = syn::parse_file(&tokens.to_string())?;
    configure_serialization(&mut syntax_tree)?;
    let items = mem::take(&mut syntax_tree.items);
    let mut formatted = prettyplease::unparse(&syntax_tree);
    if !formatted.is_empty() && !formatted.ends_with('\n') {
        formatted.push('\n');
    }
    syntax_tree.attrs.clear();
    syntax_tree.shebang = None;
    let mut previous_group = None;
    for item in items {
        let group = match &item {
            syn::Item::Use(_) => Some(ItemGroup::Imports),
            syn::Item::Mod(module) if module.content.is_none() => Some(ItemGroup::Modules),
            _ => None,
        };
        if !formatted.is_empty() && (group.is_none() || group != previous_group) {
            formatted.push('\n');
        }
        syntax_tree.items.push(item);
        formatted.push_str(&prettyplease::unparse(&syntax_tree));
        syntax_tree.items.clear();
        previous_group = group;
    }
    Ok(formatted)
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
