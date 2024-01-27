use crate::{attrs_parsing::parse_attr, stream::trim_chars};

use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    Attribute, Ident, ImplGenerics, Item, ItemEnum, ItemStruct, LitStr, Token, Type, TypeGenerics,
    WhereClause,
};

mod keywords {
    syn::custom_keyword!(key);
    syn::custom_keyword!(into);
}

/// All context attributes
/// # Fields
/// * `key` - key of context (required)
/// * `into` - type into which we need to convert context value (optional)
/// # Examples
/// ```not_rust
/// #[context(key = "a", into = Wrapper)]
/// ```
/// # Notes
/// `key` field is required, because we need to know how to find data in context
/// `into` field is optional, because we can use `From` trait to convert context value to our type
///
/// If any unknown attribute is found, then we return error
#[derive(Debug)]
struct FromContextAttrs {
    key: LitStr,
    into: Option<Type>,
}

/// Parse `#[context(...)]` attributes
/// # Examples
/// ```not_rust
/// #[context(key = "a", into = Wrapper)]
/// ```
impl Parse for FromContextAttrs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut key = None;
        let mut into = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            // If we found `,` token, then we need to skip it and continue parsing
            if lookahead.peek(Token![,]) {
                input.parse::<Token![,]>()?;

                continue;
            }

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

                // If we found `keys` attribute, then we need to skip it and continue parsing
                continue;
            }

            if lookahead.peek(keywords::into) {
                let input_into: keywords::into = input.parse()?;
                input.parse::<Token![=]>()?;

                let value: Type = input.parse()?;

                if into.is_some() {
                    return Err(syn::Error::new_spanned(
                        input_into,
                        "duplicate `into` attribute",
                    ));
                }

                into = Some(value);

                // If we found `into` attribute, then we need to skip it and continue parsing
                continue;
            }

            // If we found unknown attribute, then we need to return error
            return Err(syn::Error::new(
                input.span(),
                "expected `key` or `into` attribute",
            ));
        }

        let key = key.ok_or_else(|| syn::Error::new(input.span(), "missing `key` attribute"))?;

        Ok(Self { key, into })
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
    fn parse(_attrs: &[Attribute]) -> Result<Self, syn::Error> {
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

/// Implement `FromEventAndContext` trait for `ident` or `into` type.
/// # Arguments
/// * `ident` - type for which we need to implement `FromEventAndContext` trait if `into` field is empty
/// * `ident_impl_generics` - impl generics of `ident` type
/// * `ident_ty_generics` - type generics of `ident` type
/// * `ident_where_clause` - where clause of `ident` type
/// * `client` - client type
/// * `context_attrs` - context attributes. If `into` field is not empty,
/// then we need to implement the trait for `into` typeand require `Into<Self>` trait for `ident` type.
/// # Notes
/// Currently we can implement `FromEventAndContext` trait for `into` type only if its type generics are the same as `ident` type generics.
fn impl_from_event_and_context(
    ident: &Ident,
    ident_impl_generics: &ImplGenerics<'_>,
    ident_ty_generics: &TypeGenerics<'_>,
    ident_where_clause: Option<&WhereClause>,
    client: &Client,
    context_attrs: &FromContextAttrs,
) -> TokenStream {
    let mut impl_generics_punctuated = Punctuated::<Type, Token![,]>::new();
    let mut ty_generics_punctuated = Punctuated::<Type, Token![,]>::new();
    let mut where_clause_punctuated = Punctuated::<Type, Token![,]>::new();

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

    // Be aware that `context_key` is `LitStr`, so we need to use `value` method to get `String` instead of using `to_string` method
    let key = context_attrs.key.value();
    let key_str = key.as_str();

    // If `into` field is not empty, then we need to implement the trait for `into` type and require `Into<Self>` trait for `ident` type
    if let Some(ref into) = context_attrs.into {
        return quote_spanned! { ident.span() =>
            #[automatically_derived]
            impl <#impl_generics_punctuated> ::telers::extractors::FromEventAndContext<#client_trait_generic> for #into #ty_generics_punctuated
            where
                #where_clause_punctuated
                // `Into<#ident #ty_generics_punctuated>` is required to be able to convert context value to `into` type
                #ident #ty_generics_punctuated: ::std::clone::Clone + ::std::convert::Into<Self> + 'static
            {
                type Error = ::telers::errors::ExtractionError;

                fn extract(
                    bot: ::std::sync::Arc<::telers::client::Bot<#client_trait_generic>>,
                    update: ::std::sync::Arc<::telers::types::Update>,
                    context: ::std::sync::Arc<::telers::context::Context>,
                ) -> Result<Self, Self::Error> {
                    use ::telers::errors::ExtractionError as Error;

                    let Some(value) = context.get(#key_str) else {
                        return Err(Error::new(concat!("No found data in context by key `", #key_str, '`')));
                    };

                    match value.downcast_ref::<#ident #ty_generics_punctuated>() {
                        Some(value_ref) => Ok((*value_ref).clone().into()),
                        None => Err(Error::new(concat!(
                            "Data in context by key `", #key_str, "` has wrong type expected `", stringify!(#ident), '`',
                        ))),
                    }
                }
            }
        };
    };

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
                use ::telers::errors::ExtractionError as Error;

                let Some(value) = context.get(#key_str) else {
                    return Err(Error::new(concat!("No found data in context by key `", #key_str, '`')));
                };

                match value.downcast_ref::<Self>() {
                    Some(value_ref) => Ok((*value_ref).clone()),
                    None => Err(Error::new(concat!(
                        "Data in context by key `", #key_str, "` has wrong type expected `", stringify!(#ident), '`',
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

    let context_attrs = match parse_attr("context", attrs) {
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

    let (ident_impl_generics, ident_ty_generics, ident_where_clause) = generics.split_for_impl();

    Ok(impl_from_event_and_context(
        ident,
        &ident_impl_generics,
        &ident_ty_generics,
        ident_where_clause.as_deref(),
        &client,
        &context_attrs,
    ))
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

    let context_attrs = match parse_attr("context", attrs) {
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

    let (ident_impl_generics, ident_ty_generics, ident_where_clause) = generics.split_for_impl();

    Ok(impl_from_event_and_context(
        ident,
        &ident_impl_generics,
        &ident_ty_generics,
        ident_where_clause.as_deref(),
        &client,
        &context_attrs,
    ))
}

pub(crate) fn expand(item: Item) -> Result<TokenStream, syn::Error> {
    use Item::{Enum, Struct};

    match item {
        Struct(item) => expand_struct(&item),
        Enum(item) => expand_enum(&item),
        _ => Err(syn::Error::new_spanned(item, "expected `struct` or `enum`")),
    }
}
