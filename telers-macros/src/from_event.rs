use crate::{attrs_parsing::parse_attr, stream::trim_chars};

use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    Attribute, Ident, ImplGenerics, Item, ItemEnum, ItemStruct, LitStr, Path, Token, Type,
    TypeGenerics, WhereClause,
};

mod keywords {
    syn::custom_keyword!(from);
    syn::custom_keyword!(try_from);
    syn::custom_keyword!(description);
    syn::custom_keyword!(error);
}

/// # Notes
/// Currently, we support only `Update` type
#[derive(Debug)]
enum TypeKind {
    Update,
}

/// Parse attribute value in `#[event(from = ...)]` or `#[event(try_from = ...)]` attributes
/// # Examples
/// ```not_rusts
/// #[event(from = Update)]
/// struct Type;
///
/// #[event(try_from = Update)]
/// struct AnotherType;
/// ```
impl Parse for TypeKind {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();

        // Check if we found `Path` type
        if lookahead.peek(Ident) {
            let path: Path = input.parse()?;

            match path.segments.len() {
                1 => {
                    let segment = path.segments.first().unwrap();

                    match segment.ident.to_string().as_str() {
                        "Update" => return Ok(Self::Update),
                        _ => {
                            return Err(syn::Error::new_spanned(
                                segment,
                                "unknown type, expected `Update`",
                            ))
                        }
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        path,
                        "unknown type, expected `Update`",
                    ))
                }
            }
        }

        Err(syn::Error::new(input.span(), "expected `Update` type"))
    }
}

#[derive(Debug)]
enum ConvertKind {
    From(TypeKind),
    TryFrom(TypeKind),
}

/// All event attributes
/// # Fields
/// * `from` - type from which we need to convert event value (optional; required if `try_from` field is empty)
/// * `try_from` - type from which we need to convert event value (optional; required if `from` field is empty)
/// * `error` - type of error (optional) for `try_from`. \
/// If it's empty, then we use `ConvertToTypeError` type as error type. \
/// If it's not empty, then we use this type as error type.
/// * `description` - description of type (optional)
/// # Examples
/// ```not_rust
/// #[event(from = Update)]
/// struct Type;
///
/// #[event(try_from = Update)]
/// struct AnotherType;
/// ```
/// # Notes
/// If any unknown attribute is found, then we return error
///
/// If `try_from` is empty and `error` is not empty, then we return error
struct FromEventAttrs {
    convert_kind: ConvertKind,
    error: Option<ExtractionError>,
    _description: Option<LitStr>,
}

/// Parse `#[event(...)]` attributes
/// # Examples
/// ```not_rust
/// #[event(from = Update)]
/// ```
impl Parse for FromEventAttrs {
    #[allow(clippy::too_many_lines)]
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut convert_kind = None;
        let mut error = None;
        let mut description = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            // If we found `,` token, then we need to skip it and continue parsing
            if lookahead.peek(Token![,]) {
                input.parse::<Token![,]>()?;

                continue;
            }

            if lookahead.peek(keywords::from) {
                let input_from: keywords::from = input.parse()?;
                input.parse::<Token![=]>()?;

                let value: TypeKind = input.parse()?;

                match convert_kind {
                    Some(ConvertKind::From(_)) => {
                        return Err(syn::Error::new_spanned(
                            input_from,
                            "duplicate `from` attribute",
                        ))
                    }
                    Some(ConvertKind::TryFrom(_)) => {
                        return Err(syn::Error::new_spanned(
                            input_from,
                            "you can't use `from` and `try_from` attributes at the same time",
                        ))
                    }
                    None => {}
                }

                convert_kind = Some(ConvertKind::From(value));

                // If we found `from` attribute, then we need to skip it and continue parsing
                continue;
            }

            if lookahead.peek(keywords::try_from) {
                let input_try_from: keywords::try_from = input.parse()?;
                input.parse::<Token![=]>()?;

                let value: TypeKind = input.parse()?;

                match convert_kind {
                    Some(ConvertKind::From(_)) => {
                        return Err(syn::Error::new_spanned(
                            input_try_from,
                            "you can't use `from` and `try_from` attributes at the same time",
                        ))
                    }
                    Some(ConvertKind::TryFrom(_)) => {
                        return Err(syn::Error::new_spanned(
                            input_try_from,
                            "duplicate `try_from` attribute",
                        ))
                    }
                    None => {}
                }

                convert_kind = Some(ConvertKind::TryFrom(value));

                // If we found `try_from` attribute, then we need to skip it and continue parsing
                continue;
            }

            if lookahead.peek(keywords::error) {
                let input_error: keywords::error = input.parse()?;
                input.parse::<Token![=]>()?;

                let value: ExtractionError = input.parse()?;

                if error.is_some() {
                    return Err(syn::Error::new_spanned(
                        input_error,
                        "duplicate `error` attribute",
                    ));
                }

                error = Some(value);

                // If we found `error` attribute, then we need to skip it and continue parsing
                continue;
            }

            if lookahead.peek(keywords::description) {
                let input_description: keywords::description = input.parse()?;
                input.parse::<Token![=]>()?;

                let value: LitStr = input.parse()?;

                if description.is_some() {
                    return Err(syn::Error::new_spanned(
                        input_description,
                        "duplicate `description` attribute",
                    ));
                }

                description = Some(value);

                // If we found `description` attribute, then we need to skip it and continue parsing
                continue;
            }

            // If we found unknown attribute, then we need to return error
            return Err(syn::Error::new(
                input.span(),
                "expected `from`, `try_from` or `description` attribute",
            ));
        }

        let convert_kind = convert_kind.ok_or_else(|| {
            syn::Error::new(input.span(), "missing `from` or `try_from` attribute")
        })?;

        if let ConvertKind::From(_) = convert_kind {
            // We don't need to check `error` attribute if `from` attribute is not empty
            if error.is_some() {
                return Err(syn::Error::new(
                    input.span(),
                    "you can't use `error` attribute with `from` attribute",
                ));
            }
        } else {
            // Use default error type if `error` attribute is empty and `try_from` attribute is not empty
            if error.is_none() {
                error = Some(ExtractionError::default());
            }
        }

        Ok(Self {
            convert_kind,
            error,
            _description: description,
        })
    }
}

/// # Notes
/// Currently, we support only default client type, but in future we will support custom client types
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
    #[inline]
    const fn impl_generic(&self) -> &Type {
        match self {
            Self::Default(inner) => inner,
        }
    }

    /// ```not_rust
    /// impl<T> A<T> for B {}
    ///           ^ this type
    /// ```
    #[inline]
    const fn ty_generic(&self) -> &Type {
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

enum ExtractionError {
    Default(Type),
    Custom(Type),
}

impl ExtractionError {
    /// ```not_rust
    /// impl<T> A<T> for B {}
    ///           ^ this type
    /// ```
    fn ty_generic(&self) -> &Type {
        match self {
            Self::Default(inner) | Self::Custom(inner) => inner,
        }
    }
}

impl Default for ExtractionError {
    fn default() -> Self {
        Self::Default(parse_quote! { ::telers::errors::ConvertToTypeError })
    }
}

impl Parse for ExtractionError {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Ident) {
            let input_type: Type = input.parse()?;

            if let Type::Path(_) = input_type {
                return Ok(Self::Custom(input_type));
            }
        }

        Err(syn::Error::new(
            input.span(),
            "expected type or path to type",
        ))
    }
}

impl ToTokens for ExtractionError {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Default(inner) | Self::Custom(inner) => inner.to_tokens(tokens),
        }
    }
}

/// Implement `FromEventAndContext` trait for `ident` type.
/// # Arguments
/// * `ident` - type for which we need to implement `FromEventAndContext` trait
/// * `ident_impl_generics` - impl generics of `ident` type
/// * `ident_ty_generics` - type generics of `ident` type
/// * `ident_where_clause` - where clause of `ident` type
/// * `client` - client type
/// * `event_attrs` - event attributes
#[allow(clippy::too_many_lines)]
fn impl_from_event_and_context(
    ident: &Ident,
    ident_impl_generics: &ImplGenerics<'_>,
    ident_ty_generics: &TypeGenerics<'_>,
    ident_where_clause: Option<&WhereClause>,
    client: &Client,
    event_attrs: &FromEventAttrs,
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

    let client_ty_generic = client.ty_generic().clone();

    match &event_attrs.convert_kind {
        ConvertKind::From(TypeKind::Update) => {
            quote_spanned! { ident.span() =>
                #[automatically_derived]
                impl <#impl_generics_punctuated> ::telers::extractors::FromEventAndContext<#client_ty_generic> for #ident #ty_generics_punctuated
                where
                    #where_clause_punctuated
                    ::telers::types::Update: ::std::convert::Into<Self>
                {
                    type Error = ::std::convert::Infallible;

                    #[inline]
                    fn extract(
                        bot: ::std::sync::Arc<::telers::client::Bot<#client_ty_generic>>,
                        update: ::std::sync::Arc<::telers::types::Update>,
                        context: ::std::sync::Arc<::telers::context::Context>,
                    ) -> Result<Self, Self::Error> {
                        Ok((*update).clone().into())
                    }
                }
            }
        }
        ConvertKind::TryFrom(TypeKind::Update) => {
            let error = event_attrs
                .error
                .as_ref()
                .expect("error is empty in `try_from`, but it should be filled automatically");
            let error_ty = error.ty_generic().clone();

            quote_spanned! { ident.span() =>
                #[automatically_derived]
                impl <#impl_generics_punctuated> ::telers::extractors::FromEventAndContext<#client_ty_generic> for #ident #ty_generics_punctuated
                where
                    #where_clause_punctuated
                    ::telers::types::Update: ::std::convert::TryInto<Self>
                {
                    type Error = #error_ty;

                    #[inline]
                    fn extract(
                        bot: ::std::sync::Arc<::telers::client::Bot<#client_ty_generic>>,
                        update: ::std::sync::Arc<::telers::types::Update>,
                        context: ::std::sync::Arc<::telers::context::Context>,
                    ) -> Result<Self, Self::Error> {
                        ::std::convert::TryFrom::try_from((*update).clone())
                    }
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

    let event_attrs = match parse_attr("event", attrs) {
        Ok(Some(attrs)) => attrs,
        Ok(None) => {
            return Err(syn::Error::new_spanned(
                ident,
                "missing `#[event(...)]` attribute",
            ))
        }
        Err(err) => {
            return Err(syn::Error::new_spanned(
                ident,
                format!("failed to parse `#[event(...)]` attributes: {err}"),
            ))
        }
    };

    let (ident_impl_generics, ident_ty_generics, ident_where_clause) = generics.split_for_impl();

    Ok(impl_from_event_and_context(
        ident,
        &ident_impl_generics,
        &ident_ty_generics,
        ident_where_clause,
        &client,
        &event_attrs,
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

    let event_attrs = match parse_attr("event", attrs) {
        Ok(Some(attrs)) => attrs,
        Ok(None) => {
            return Err(syn::Error::new_spanned(
                ident,
                "missing `#[event(...)]` attribute",
            ))
        }
        Err(err) => {
            return Err(syn::Error::new_spanned(
                ident,
                format!("failed to parse `#[event(...)]` attributes: {err}"),
            ))
        }
    };

    let (ident_impl_generics, ident_ty_generics, ident_where_clause) = generics.split_for_impl();

    Ok(impl_from_event_and_context(
        ident,
        &ident_impl_generics,
        &ident_ty_generics,
        ident_where_clause,
        &client,
        &event_attrs,
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
