//! This module contains helpers for parsing of the attributes of the macros

use quote::ToTokens;
use syn::{meta::ParseNestedMeta, Attribute, Path};

/// Parses the `#[name(...)]` attribute of the item
/// # Arguments
/// * `name` - Name of the attribute
/// * `attrs` - Attributes of the item
/// * `parse_meta` - Parser of an argument of the attribute (`name = value` pair or flag), called for each of them
/// # Returns
/// The attribute for the spans of further errors, `None` if the item has no such attribute
/// # Errors
/// - If the attribute is repeated
/// - If `parse_meta` fails
pub(crate) fn parse_attr<'a>(
    name: &str,
    attrs: &'a [Attribute],
    parse_meta: impl FnMut(ParseNestedMeta<'_>) -> syn::Result<()>,
) -> syn::Result<Option<&'a Attribute>> {
    let mut attrs = attrs.iter().filter(|attr| attr.path().is_ident(name));
    let Some(attr) = attrs.next() else {
        return Ok(None);
    };
    if let Some(duplicate) = attrs.next() {
        return Err(syn::Error::new_spanned(
            duplicate,
            format!("duplicate `{name}` attribute"),
        ));
    }

    attr.parse_nested_meta(parse_meta)?;
    Ok(Some(attr))
}

/// Sets the value of an argument of an attribute
/// # Arguments
/// * `arg` - Value of the argument
/// * `path` - Name of the argument for the error
/// * `val` - Value to set
/// # Errors
/// - If the argument is already set
pub(crate) fn set_once<T>(arg: &mut Option<T>, path: &Path, val: T) -> syn::Result<()> {
    if arg.is_some() {
        return Err(syn::Error::new_spanned(
            path,
            format!("duplicate `{}` attribute", path.to_token_stream()),
        ));
    }
    *arg = Some(val);
    Ok(())
}
