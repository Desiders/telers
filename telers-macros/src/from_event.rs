use crate::{
    attrs_parsing::{parse_attr, set_once},
    extractor::{extractor_generics, tokenize_extractor_impl},
};

use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::{meta::ParseNestedMeta, parse_quote, Attribute, Data, DeriveInput, LitStr, Path, Type};

/// Conversion of the event into the type
enum ConvertKind {
    /// `From<Update>`
    From,
    /// `TryFrom<Update>` with the error type of the conversion
    TryFrom { error: Box<Type> },
}

/// `#[event(...)]` attributes
/// # Fields
/// * `from` / `try_from` - `Update`, the type from which the type is converted (one of them is required)
/// * `error` - error type of the conversion for `try_from` (optional, `ConvertToTypeError` by default)
/// * `description` - description of the type, accepted for documentation only (optional)
/// # Examples
/// ```not_rust
/// #[event(from = Update)]
/// struct Type;
///
/// #[event(try_from = Update, error = Infallible)]
/// struct AnotherType;
/// ```
struct FromEventAttrs {
    convert_kind: ConvertKind,
}

impl FromEventAttrs {
    /// Parses `#[event(from = Update)]`, `None` if the item has no such attribute
    fn parse(attrs: &[Attribute]) -> syn::Result<Option<Self>> {
        let (mut from, mut try_from, mut error, mut description) = (None, None, None, None);
        let Some(attr) = parse_attr("event", attrs, |meta| {
            if meta.path.is_ident("from") {
                parse_event_type(&meta)?;
                set_once(&mut from, &meta.path, ())
            } else if meta.path.is_ident("try_from") {
                parse_event_type(&meta)?;
                set_once(&mut try_from, &meta.path, ())
            } else if meta.path.is_ident("error") {
                let val = meta.value()?.parse()?;
                set_once(&mut error, &meta.path, val)
            } else if meta.path.is_ident("description") {
                let val = meta.value()?.parse::<LitStr>()?;
                set_once(&mut description, &meta.path, val)
            } else {
                Err(meta.error("expected `from`, `try_from`, `error` or `description` attribute"))
            }
        })?
        else {
            return Ok(None);
        };

        let convert_kind = match (from, try_from) {
            (Some(()), Some(())) => {
                return Err(syn::Error::new_spanned(
                    attr,
                    "you can't use `from` and `try_from` attributes at the same time",
                ));
            }
            (Some(()), None) => {
                if error.is_some() {
                    return Err(syn::Error::new_spanned(
                        attr,
                        "you can't use `error` attribute with `from` attribute",
                    ));
                }
                ConvertKind::From
            }
            (None, Some(())) => ConvertKind::TryFrom {
                error: error
                    .unwrap_or_else(|| parse_quote! { ::telers::errors::ConvertToTypeError }),
            },
            (None, None) => {
                return Err(syn::Error::new_spanned(
                    attr,
                    "missing `from` or `try_from` attribute",
                ));
            }
        };

        Ok(Some(Self {
            convert_kind,
        }))
    }
}

/// Parses the type of the event in `from = ...` or `try_from = ...`, only `Update` is supported
fn parse_event_type(meta: &ParseNestedMeta<'_>) -> syn::Result<()> {
    let path: Path = meta.value()?.parse()?;
    if path.is_ident("Update") {
        Ok(())
    } else {
        Err(syn::Error::new_spanned(
            path,
            "unknown type, expected `Update`",
        ))
    }
}

/// Implements `Extractor` for the type, which converts the update of the event into it
pub(crate) fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput {
        attrs,
        ident,
        generics,
        data,
        ..
    } = input;
    if let Data::Union(_) = data {
        return Err(syn::Error::new_spanned(
            ident,
            "expected `struct` or `enum`",
        ));
    }
    let attrs = FromEventAttrs::parse(&attrs)?
        .ok_or_else(|| syn::Error::new_spanned(&ident, "missing `#[event(...)]` attribute"))?;

    let (_, ty_generics, _) = generics.split_for_impl();
    let self_ty = quote_spanned! { ident.span() => #ident #ty_generics };
    let (error, convert_bound, body) = match attrs.convert_kind {
        ConvertKind::From => (
            quote_spanned! { ident.span() => ::std::convert::Infallible },
            parse_quote! { ::telers::types::Update: ::std::convert::Into<Self> },
            quote_spanned! { ident.span() =>
                let val = (*request.update).clone().into();
                async move { ::std::result::Result::Ok(val) }
            },
        ),
        ConvertKind::TryFrom {
            error,
        } => (
            error.into_token_stream(),
            parse_quote! { ::telers::types::Update: ::std::convert::TryInto<Self> },
            quote_spanned! { ident.span() =>
                let val = ::std::convert::TryFrom::try_from((*request.update).clone());
                async move { val }
            },
        ),
    };
    let generics = extractor_generics(
        &generics,
        [
            parse_quote! { #self_ty: ::std::marker::Send },
            convert_bound,
        ],
    );

    Ok(tokenize_extractor_impl(
        ident.span(),
        &self_ty,
        &generics,
        &error,
        &body,
    ))
}
