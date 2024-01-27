use crate::{attrs_parsing::parse_attr, stream::trim_chars};

use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    Generics, Ident, Item, ItemEnum, ItemStruct, LitStr, Token, Type,
};

mod keywords {
    syn::custom_keyword!(key);
}

#[derive(Debug)]
struct FromContextAttrs {
    key: LitStr,
}

impl Parse for FromContextAttrs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut key = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            if lookahead.peek(keywords::key) {
                let input_key: keywords::key = input.parse()?;
                input.parse::<Token![=]>()?;

                let value: LitStr = input.parse()?;

                if key.is_some() {
                    return Err(syn::Error::new_spanned(
                        input_key,
                        "duplicate `key` attribute",
                    ));
                }

                key = Some(value);
            } else {
                return Err(lookahead.error());
            }
        }

        let key = key.ok_or_else(|| syn::Error::new(input.span(), "missing `key` attribute"))?;

        Ok(Self { key })
    }
}

/// # Notes
/// Currently, we support only default client type, but in future we will support custom client types
#[derive(Debug)]
enum Client {
    Default(Type),
}

impl Client {
    // # Notes
    // Currently, we support only default client type, but in future we will support custom client types
    #[allow(clippy::unnecessary_wraps, clippy::needless_pass_by_value)]
    fn parse(_attrs: &[syn::Attribute]) -> Result<Self, syn::Error> {
        // We use `__` prefix here to avoid name conflicts
        let path = parse_quote! { __C };

        Ok(Self::Default(path))
    }

    /// ```not_rust
    /// impl<T> A for B {}
    ///      ^ this type
    /// ```
    fn impl_generic(&self) -> &Type {
        match self {
            Self::Default(inner) => inner,
        }
    }

    /// ```not_rust
    /// impl<T> A<T> for B {}
    ///           ^ this type
    /// ```
    fn trait_generic(&self) -> &Type {
        match self {
            Self::Default(inner) => inner,
        }
    }
}

impl ToTokens for Client {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Default(inner) => inner.to_tokens(tokens),
        }
    }
}

fn impl_from_event_and_context(
    ident: &Ident,
    ident_generics: &Generics,
    client: &Client,
    context_key: &LitStr,
) -> TokenStream {
    let mut impl_generics_punctuated = Punctuated::<Type, Token![,]>::new();
    let mut ty_generics_punctuated = Punctuated::<Type, Token![,]>::new();
    let mut where_clause_punctuated = Punctuated::<Type, Token![,]>::new();

    let (ident_impl_generics, ident_ty_generics, ident_where_clause) =
        ident_generics.split_for_impl();

    // If impl generics is not empty, then we need to remove first token (usually it is `<`)
    // and last token (usually it is `>`), because we need to add our generic type to it.
    // Example: `<T, E>, OUR_GENERIC` => `T, E, OUR_GENERIC`. (check `trim_chars` tests for more examples)
    // I don't know how to do it better.
    if !ident_impl_generics.to_token_stream().is_empty() {
        // Stream without `<` and `>` chars as last and first tokens
        let stream = trim_chars(ident_impl_generics.to_token_stream(), Some('<'), Some('>'));
        // Stream without `,` char as last token
        let stream = trim_chars(stream, None, Some(','));

        impl_generics_punctuated.push(Type::Verbatim(stream));
    }

    impl_generics_punctuated.push(client.impl_generic().clone());
    ty_generics_punctuated.push(Type::Verbatim(ident_ty_generics.into_token_stream()));
    where_clause_punctuated.push(Type::Verbatim(ident_where_clause.into_token_stream()));

    let client_trait_generic = client.trait_generic().clone();

    let key = context_key.token().to_string();
    let key_str = key.as_str();

    quote_spanned! { ident.span() =>
        #[automatically_derived]
        impl <#impl_generics_punctuated> ::telers::extractors::FromEventAndContext<#client_trait_generic> for #ident #ty_generics_punctuated
        where
            #where_clause_punctuated
            #ident #ty_generics_punctuated: ::std::clone::Clone + 'static
        {
            type Error = ::telers::errors::ExtractionError;

            fn extract(
                bot: ::std::sync::Arc<::telers::client::Bot<#client_trait_generic>>,
                update: ::std::sync::Arc<::telers::types::Update>,
                context: ::std::sync::Arc<::telers::context::Context>,
            ) -> Result<Self, Self::Error> {
                let Some(value) = context.get(#key_str) else {
                    return Err(::telers::errors::ExtractionError::new(concat!("No found data in context by key ", #key_str)));
                };

                match value.downcast_ref::<Self>() {
                    Some(value_ref) => Ok((*value_ref).clone()),
                    None => Err(::telers::errors::ExtractionError::new(concat!(
                        "Data in context by key ",
                        #key_str,
                        " has wrong type expected ",
                        stringify!(#ident),
                    ))),
                }
            }
        }
    }
}

fn expand_struct(
    ItemStruct {
        attrs,
        ident,
        generics,
        ..
    }: &ItemStruct,
) -> Result<TokenStream, syn::Error> {
    let client = match Client::parse(attrs) {
        Ok(client) => client,
        Err(err) => {
            return Err(syn::Error::new_spanned(
                ident,
                format!("failed to parse attributes: {err}"),
            ))
        }
    };

    let FromContextAttrs { key } = match parse_attr("context", attrs) {
        Ok(Some(attrs)) => attrs,
        Ok(None) => {
            return Err(syn::Error::new_spanned(
                ident,
                "missing `#[context(...)]` attribute",
            ))
        }
        Err(err) => {
            return Err(syn::Error::new_spanned(
                ident,
                format!("failed to parse `#[context(...)]` attributes: {err}"),
            ))
        }
    };

    Ok(impl_from_event_and_context(ident, generics, &client, &key))
}

fn expand_enum(
    ItemEnum {
        attrs,
        ident,
        generics,
        ..
    }: &ItemEnum,
) -> Result<TokenStream, syn::Error> {
    let client = match Client::parse(attrs) {
        Ok(client) => client,
        Err(err) => {
            return Err(syn::Error::new_spanned(
                ident,
                format!("failed to parse attributes: {err}"),
            ))
        }
    };

    let FromContextAttrs { key } = match parse_attr("context", attrs) {
        Ok(Some(attrs)) => attrs,
        Ok(None) => {
            return Err(syn::Error::new_spanned(
                ident,
                "missing `#[context(...)]` attribute",
            ))
        }
        Err(err) => {
            return Err(syn::Error::new_spanned(
                ident,
                format!("failed to parse `#[context(...)]` attributes: {err}"),
            ))
        }
    };

    Ok(impl_from_event_and_context(ident, generics, &client, &key))
}

pub(crate) fn expand(item: Item) -> Result<TokenStream, syn::Error> {
    use Item::{Enum, Struct};

    match item {
        Struct(item) => expand_struct(&item),
        Enum(item) => expand_enum(&item),
        _ => Err(syn::Error::new_spanned(item, "expected `struct` or `enum`")),
    }
}
