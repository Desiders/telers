use crate::{
    attrs_parsing::{parse_attr, set_once},
    extractor::{extractor_generics, tokenize_extractor_impl},
};

use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::{parse_quote, Attribute, Data, DeriveInput, LitStr, Type};

/// `#[context(...)]` attributes
/// # Fields
/// * `key` - key of the value in the context (required)
/// * `into` - type the value is converted into, the `Extractor` is implemented for it (optional)
/// * `from` - type of the value in the context which is converted into the type (optional)
/// * `description` - description of the type, it's a part of the extraction error (optional)
/// # Examples
/// ```not_rust
/// #[context(key = "type", into = TypeWrapper)]
/// struct Type;
///
/// #[context(key = "type", from = Type)]
/// struct TypeWrapper(Type);
/// ```
struct FromContextAttrs {
    key: LitStr,
    into: Option<Type>,
    from: Option<Type>,
    description: Option<LitStr>,
}

impl FromContextAttrs {
    /// Parses `#[context(key = "a", into = Wrapper)]`, `None` if the item has no such attribute
    fn parse(attrs: &[Attribute]) -> syn::Result<Option<Self>> {
        let (mut key, mut into, mut from, mut description) = (None, None, None, None);
        let Some(attr) = parse_attr("context", attrs, |meta| {
            if meta.path.is_ident("key") {
                let val = meta.value()?.parse()?;
                set_once(&mut key, &meta.path, val)
            } else if meta.path.is_ident("into") {
                let val = meta.value()?.parse()?;
                set_once(&mut into, &meta.path, val)
            } else if meta.path.is_ident("from") {
                let val = meta.value()?.parse()?;
                set_once(&mut from, &meta.path, val)
            } else if meta.path.is_ident("description") {
                let val = meta.value()?.parse()?;
                set_once(&mut description, &meta.path, val)
            } else {
                Err(meta.error("expected `key`, `into`, `from` or `description` attribute"))
            }
        })?
        else {
            return Ok(None);
        };

        let key = key.ok_or_else(|| syn::Error::new_spanned(attr, "missing `key` attribute"))?;
        if into.is_some() && from.is_some() {
            return Err(syn::Error::new_spanned(
                attr,
                "you can't use `into` and `from` attributes at the same time",
            ));
        }

        Ok(Some(Self {
            key,
            into,
            from,
            description,
        }))
    }
}

/// Implements `Extractor` for the type, or for the `into` type if it's set,
/// which gets the value from the context by the key and converts it
/// # Notes
/// The converted type must have the same generics as the type
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
    let attrs = FromContextAttrs::parse(&attrs)?
        .ok_or_else(|| syn::Error::new_spanned(&ident, "missing `#[context(...)]` attribute"))?;

    let (_, ty_generics, _) = generics.split_for_impl();
    // The value in the context is `source_ty`, the impl is for the type it's converted into,
    // both are the type itself if there is no conversion
    let self_ty = attrs.into.as_ref().map_or_else(
        || quote_spanned! { ident.span() => #ident #ty_generics },
        |into| quote_spanned! { ident.span() => #into #ty_generics },
    );
    let (source_ty, expected) = match &attrs.from {
        Some(from) => (
            quote_spanned! { ident.span() => #from #ty_generics },
            from.to_token_stream().to_string(),
        ),
        None => (
            quote_spanned! { ident.span() => #ident #ty_generics },
            ident.to_string(),
        ),
    };
    let converts = attrs.into.is_some() || attrs.from.is_some();
    let conversion = converts.then(|| quote_spanned! { ident.span() => .into() });
    let into_bound =
        converts.then(|| quote_spanned! { ident.span() => + ::std::convert::Into<Self> });

    let key = &attrs.key;
    let description = attrs
        .description
        .as_ref()
        .map_or_else(|| "no description".to_owned(), LitStr::value);
    let msg = format!(
        "No found data in context by key `{}` or value has wrong type expected `{expected}`. You \
         didn't forget to add type to context? Type description: {description}",
        key.value(),
    );

    let body = quote_spanned! { ident.span() =>
        let res = match request.context.get::<#source_ty>(#key) {
            ::std::option::Option::Some(value) => ::std::result::Result::Ok((*value).clone() #conversion),
            ::std::option::Option::None => ::std::result::Result::Err(
                ::telers::errors::ExtractionError::new(#msg),
            ),
        };
        async move { res }
    };
    let generics = extractor_generics(
        &generics,
        [parse_quote! {
            #source_ty: ::std::clone::Clone #into_bound + ::std::marker::Send + 'static
        }],
    );
    let error = quote_spanned! { ident.span() => ::telers::errors::ExtractionError };

    Ok(tokenize_extractor_impl(
        ident.span(),
        &self_ty,
        &generics,
        &error,
        &body,
    ))
}
