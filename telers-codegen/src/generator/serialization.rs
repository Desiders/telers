//! Keep the native Deser derives in sync with the generated Serde wire format.

use quote::quote;
use syn::{Attribute, Fields, Item, Meta, Token, parse_quote, punctuated::Punctuated};

/// Select Serde or native Deser implementations for serializable top-level types.
///
/// Serde derives and wire attributes are disabled when Deser is selected.
///
/// # Errors
/// Returns an error if a derive or wire attribute cannot be parsed.
pub fn configure_serialization(file: &mut syn::File) -> syn::Result<()> {
    for item in &mut file.items {
        let (attrs, fields): (&mut Vec<Attribute>, Vec<&mut Fields>) = match item {
            Item::Struct(item) => (&mut item.attrs, vec![&mut item.fields]),
            Item::Enum(item) => (
                &mut item.attrs,
                item.variants.iter_mut().map(|v| &mut v.fields).collect(),
            ),
            _ => continue,
        };
        let mut derives = Vec::new();
        let mut serde_derives = Vec::new();
        for attr in attrs.iter().filter(|a| a.path().is_ident("derive")) {
            let paths =
                attr.parse_args_with(Punctuated::<syn::Path, Token![,]>::parse_terminated)?;
            for path in paths {
                match path.segments.last().map(|s| s.ident.to_string()).as_deref() {
                    Some("Serialize") => {
                        derives.push(quote!(deser::Serialize));
                        serde_derives.push(quote!(serde::Serialize));
                    }
                    Some("Deserialize") => {
                        derives.push(quote!(deser::Deserialize));
                        serde_derives.push(quote!(serde::Deserialize));
                    }
                    _ => {}
                }
            }
        }
        if derives.is_empty() {
            continue;
        }
        for attr in attrs.iter_mut().filter(|a| a.path().is_ident("derive")) {
            let paths =
                attr.parse_args_with(Punctuated::<syn::Path, Token![,]>::parse_terminated)?;
            let paths: Punctuated<syn::Path, Token![,]> = paths
                .into_iter()
                .filter(|p| {
                    !matches!(
                        p.segments.last().map(|s| s.ident.to_string()).as_deref(),
                        Some("Serialize" | "Deserialize")
                    )
                })
                .collect();
            *attr = parse_quote!(#[derive(#paths)]);
        }
        attrs.retain(|attr| {
            !(attr.path().is_ident("derive")
                && attr
                    .meta
                    .require_list()
                    .is_ok_and(|list| list.tokens.is_empty()))
        });
        let wire_index = attrs
            .iter()
            .position(|attr| attr.path().is_ident("serde"))
            .unwrap_or(attrs.len());
        attrs.insert(
            wire_index,
            parse_quote!(#[cfg_attr(feature = "deser", derive(#(#derives),*))]),
        );
        attrs.insert(
            wire_index + 1,
            parse_quote!(#[cfg_attr(not(feature = "deser"), derive(#(#serde_derives),*))]),
        );
        copy_wire_attributes(attrs)?;
        for fields in fields {
            for field in fields.iter_mut() {
                copy_wire_attributes(&mut field.attrs)?;
            }
        }
        if let Item::Enum(item) = item {
            for variant in &mut item.variants {
                copy_wire_attributes(&mut variant.attrs)?;
            }
        }
    }
    Ok(())
}

fn copy_wire_attributes(attrs: &mut Vec<Attribute>) -> syn::Result<()> {
    let mut added = Vec::new();
    for attr in attrs.iter().filter(|a| a.path().is_ident("serde")) {
        let mut options = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
        for option in &mut options {
            // Deser takes a Rust path, whereas Serde takes its string spelling.
            if let Meta::NameValue(value) = option
                && value.path.is_ident("skip_serializing_if")
                && let syn::Expr::Lit(expr) = &value.value
                && let syn::Lit::Str(path) = &expr.lit
            {
                value.value = syn::parse_str(&path.value())?;
            }
        }
        added.push(parse_quote!(#[cfg_attr(feature = "deser", deser(#options))]));
    }
    for attr in attrs.iter_mut().filter(|a| a.path().is_ident("serde")) {
        let meta = &attr.meta;
        *attr = parse_quote!(#[cfg_attr(not(feature = "deser"), #meta)]);
    }
    attrs.extend(added);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_tagged_enums_and_converts_predicates_to_paths() {
        let mut file: syn::File = parse_quote! {
            #[derive(serde::Serialize, serde::Deserialize)]
            #[serde(tag = "type", rename_all = "snake_case")]
            enum Example {
                Known {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    value: Option<String>,
                },
                #[serde(untagged)]
                Other(String),
            }
        };
        configure_serialization(&mut file).unwrap();
        let actual = quote!(#file).to_string();
        assert!(actual.contains("derive (deser :: Serialize , deser :: Deserialize)"));
        assert!(actual.contains("deser (tag = \"type\" , rename_all = \"snake_case\")"));
        assert!(actual.contains("deser (skip_serializing_if = Option :: is_none)"));
        assert!(actual.contains("deser (untagged)"));
        // Helpers must follow their derives in both feature configurations.
        assert!(actual.find("derive (serde ::").unwrap() < actual.find("serde (tag").unwrap());
        assert!(actual.find("derive (deser ::").unwrap() < actual.find("deser (tag").unwrap());
    }

    #[test]
    fn unknown_json_payloads_use_native_flatten() {
        let mut file: syn::File = parse_quote! {
            #[derive(Serialize, Deserialize)]
            struct Unknown {
                #[serde(flatten)]
                extra: BTreeMap<Box<str>, crate::serialization::Value>,
            }
        };
        configure_serialization(&mut file).unwrap();
        let actual = quote!(#file).to_string();
        assert!(!actual.contains("deser_serde"));
        assert!(actual.contains("deser (flatten)"));
    }
}
