use crate::{attrs_parsing::parse_attr, stream::trim_chars};

use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    Attribute, Ident, Item, ItemStruct, LitChar, LitStr, Token, Type,
};

mod keywords {
    syn::custom_keyword!(prefix);
    syn::custom_keyword!(separator);
}

/// All callback data attributes
/// # Fields
/// * `prefix` - prefix of callback data (required)
/// * `separator` - separator of callback data values (optional, `:` by default)
/// # Examples
/// ```not_rust
/// #[callback_data(prefix = "language")]
/// struct Language;
///
/// #[callback_data(prefix = "language", separator = '|')]
/// struct Language2;
/// ```
/// # Notes
/// If any unknown attribute is found, then we return error
struct CallbackDataAttrs {
    prefix: LitStr,
    separator: Option<LitChar>,
}

/// Parse `#[callback_data(...)]` attributes
/// # Examples
/// ```not_rust
/// #[callback_data(prefix = "a", separator = '|')]
/// ```
impl Parse for CallbackDataAttrs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut prefix = None;
        let mut separator = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            // If we found `,` token, then we need to skip it and continue parsing
            if lookahead.peek(Token![,]) {
                input.parse::<Token![,]>()?;

                continue;
            }

            if lookahead.peek(keywords::prefix) {
                let input_prefix: keywords::prefix = input.parse()?;
                input.parse::<Token![=]>()?;

                let value: LitStr = input.parse()?;

                if prefix.is_some() {
                    return Err(syn::Error::new_spanned(
                        input_prefix,
                        "duplicate `prefix` attribute",
                    ));
                }

                prefix = Some(value);

                // If we found `prefix` attribute, then we need to skip it and continue parsing
                continue;
            }

            if lookahead.peek(keywords::separator) {
                let input_separator: keywords::separator = input.parse()?;
                input.parse::<Token![=]>()?;

                let value: LitChar = input.parse()?;

                if separator.is_some() {
                    return Err(syn::Error::new_spanned(
                        input_separator,
                        "duplicate `separator` attribute",
                    ));
                }

                separator = Some(value);

                // If we found `separator` attribute, then we need to skip it and continue parsing
                continue;
            }

            // If we found unknown attribute, then we need to return error
            return Err(syn::Error::new(
                input.span(),
                "expected `prefix` or `separator` attribute",
            ));
        }

        let prefix =
            prefix.ok_or_else(|| syn::Error::new(input.span(), "missing `prefix` attribute"))?;

        Ok(Self {
            prefix,
            separator,
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

/// Expand `#[derive(CallbackData)]` for the struct:
/// generate `pack` and `unpack` methods, `CallbackData` implementation
/// and `Extractor` implementation to extract the unpacked data from context.
/// # Errors
/// If the item is not a struct or `#[callback_data(...)]` attributes are invalid
#[allow(clippy::too_many_lines)]
pub(crate) fn expand(item: Item) -> Result<TokenStream, syn::Error> {
    let item = match item {
        Item::Struct(item) => item,
        _ => {
            return Err(syn::Error::new_spanned(
                item,
                "`CallbackData` can be derived only for `struct`",
            ))
        }
    };

    let ItemStruct {
        attrs,
        ident,
        generics,
        fields,
        ..
    } = &item;

    let callback_data_attrs: CallbackDataAttrs = match parse_attr("callback_data", attrs) {
        Ok(Some(attrs)) => attrs,
        Ok(None) => {
            return Err(syn::Error::new_spanned(
                ident,
                "missing `#[callback_data(...)]` attribute",
            ))
        }
        Err(err) => {
            return Err(syn::Error::new_spanned(
                ident,
                format!("failed to parse `#[callback_data(...)]` attributes: {err}"),
            ))
        }
    };

    let client = Client::parse(attrs)?;

    let prefix = &callback_data_attrs.prefix;
    // `:` by default, check `DEFAULT_SEPARATOR` in `telers` crate
    let separator = callback_data_attrs
        .separator
        .as_ref()
        .map(LitChar::value)
        .unwrap_or(':');
    if fields.is_empty() {
        return Err(syn::Error::new_spanned(
            ident,
            "`CallbackData` can't be derived for structs without fields",
        ));
    }

    let field_idents = fields
        .iter()
        .map(|field| {
            field.ident.clone().ok_or_else(|| {
                syn::Error::new_spanned(
                    field,
                    "`CallbackData` can't be derived for structs without named fields",
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let field_tys = fields.iter().map(|field| &field.ty);
    let field_names = field_idents.iter().map(Ident::to_string);
    let field_count = field_idents.len();

    let (ident_impl_generics, ident_ty_generics, ident_where_clause) = generics.split_for_impl();

    let mut impl_generics_punctuated = Punctuated::<Type, Token![,]>::new();
    let mut ty_generics_punctuated = Punctuated::<Type, Token![,]>::new();
    let mut where_clause_punctuated = Punctuated::<Type, Token![,]>::new();

    // If impl generics is not empty, then we need to remove first token (usually it is `<`)
    // and last token (usually it is `>`), because we need to add our generic type to it.
    // Example: `<T, E>, OUR_GENERIC` => `T, E, OUR_GENERIC`. (check `trim_chars` tests for more examples)
    if !ident_impl_generics.to_token_stream().is_empty() {
        let stream = trim_chars(ident_impl_generics.to_token_stream(), Some('<'), Some('>'));
        let stream = trim_chars(stream, None, Some(','));

        impl_generics_punctuated.push(Type::Verbatim(stream));
    }

    impl_generics_punctuated.push(client.impl_generic().clone());
    ty_generics_punctuated.push(Type::Verbatim(
        ident_ty_generics.clone().into_token_stream(),
    ));

    // Splice only the *predicates* of the type's `where` clause: `WhereClause::to_tokens` would
    // also emit its `where` keyword, and the impl templates below already contain a literal `where`
    // (which would expand to an unparsable `where where ...`).
    if let Some(where_clause) = ident_where_clause {
        for predicate in &where_clause.predicates {
            where_clause_punctuated.push_value(Type::Verbatim(predicate.to_token_stream()));
            where_clause_punctuated.push_punct(<Token![,]>::default());
        }
    }

    let client_ty_generic = client.ty_generic().clone();

    Ok(quote_spanned! { ident.span() =>
        #[automatically_derived]
        impl #ident_impl_generics ::telers::callback_data::CallbackData for #ident #ident_ty_generics #ident_where_clause
        {
            const PREFIX: &'static str = #prefix;
            const SEPARATOR: char = #separator;

            /// Packs the struct to a callback data string
            ///
            /// # Errors
            /// - If a value contains the separator character
            /// - If the resulting string is longer than [`MAX_CALLBACK_LENGTH`](::telers::callback_data::MAX_CALLBACK_LENGTH) bytes
            #[inline]
            fn pack(&self) -> ::std::result::Result<::std::string::String, ::telers::callback_data::CallbackDataError> {
                ::telers::callback_data::pack_values(
                    Self::PREFIX,
                    Self::SEPARATOR,
                    &[
                        #(::telers::callback_data::CallbackDataValue::encode(&self.#field_idents),)*
                    ],
                )
            }

            /// Unpacks the callback data string to the struct
            ///
            /// # Errors
            /// - If the prefix of the callback data string doesn't match
            /// - If the number of values doesn't match the number of fields
            /// - If a value can't be parsed to the field type
            #[inline]
            fn unpack(value: &str) -> ::std::result::Result<Self, ::telers::callback_data::CallbackDataError> {
                let values = ::telers::callback_data::unpack_values(value, Self::PREFIX, Self::SEPARATOR, #field_count)?;
                let mut values = values.into_vec().into_iter();

                #(
                    let #field_idents = <#field_tys as ::telers::callback_data::CallbackDataValue>::decode(
                        values.next().unwrap_or_default(),
                        #field_names,
                    )?;
                )*

                ::std::result::Result::Ok(Self {
                    #(#field_idents,)*
                })
            }
        }

        #[automatically_derived]
        impl<#impl_generics_punctuated> ::telers::Extractor<#client_ty_generic> for #ident #ty_generics_punctuated
        where
            #where_clause_punctuated
            #ident #ty_generics_punctuated: ::std::clone::Clone + Send + 'static,
        {
            type Error = ::telers::errors::ExtractionError;

            #[inline]
            fn extract(request: &::telers::Request<#client_ty_generic>) -> impl ::std::future::Future<Output = ::std::result::Result<Self, Self::Error>> + Send {
                use ::telers::errors::ExtractionError as Error;

                let res = match request.context.get::<#ident #ty_generics_punctuated>("callback_data") {
                    ::std::option::Option::Some(value) => ::std::result::Result::Ok((*value).clone()),
                    ::std::option::Option::None => ::std::result::Result::Err(Error::new(concat!(
                        "No found data in context by key `callback_data` or value has wrong type expected `", stringify!(#ident), "`. ",
                        "You didn't forget to add the `CallbackDataFilter` filter to the handler?",
                    ))),
                };
                async move { res }
            }
        }
    })
}
