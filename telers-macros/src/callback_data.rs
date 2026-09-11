use crate::{
    attrs_parsing::{parse_attr, set_once},
    extractor::{extractor_generics, tokenize_extractor_impl},
};

use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{parse_quote, Attribute, Data, DeriveInput, Fields, LitChar, LitStr};

/// `#[callback_data(...)]` attributes
/// # Fields
/// * `prefix` - prefix of the callback data (required)
/// * `separator` - separator of the callback data values (optional, `:` by default)
/// # Examples
/// ```not_rust
/// #[callback_data(prefix = "language")]
/// struct Language;
///
/// #[callback_data(prefix = "language", separator = '|')]
/// struct Language2;
/// ```
struct CallbackDataAttrs {
    prefix: LitStr,
    separator: Option<LitChar>,
}

impl CallbackDataAttrs {
    /// Parses `#[callback_data(prefix = "a", separator = '|')]`, `None` if the item has no such attribute
    fn parse(attrs: &[Attribute]) -> syn::Result<Option<Self>> {
        let (mut prefix, mut separator) = (None, None);
        let Some(attr) = parse_attr("callback_data", attrs, |meta| {
            if meta.path.is_ident("prefix") {
                let val = meta.value()?.parse()?;
                set_once(&mut prefix, &meta.path, val)
            } else if meta.path.is_ident("separator") {
                let val = meta.value()?.parse()?;
                set_once(&mut separator, &meta.path, val)
            } else {
                Err(meta.error("expected `prefix` or `separator` attribute"))
            }
        })?
        else {
            return Ok(None);
        };

        let prefix =
            prefix.ok_or_else(|| syn::Error::new_spanned(attr, "missing `prefix` attribute"))?;

        Ok(Some(Self {
            prefix,
            separator,
        }))
    }
}

/// Implements `CallbackData` for the struct with `pack` and `unpack` of its fields
/// and `Extractor`, which gets the unpacked data from the context
/// # Errors
/// If the item is not a struct with named fields or `#[callback_data(...)]` attributes are invalid
pub(crate) fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput {
        attrs,
        ident,
        generics,
        data,
        ..
    } = input;
    let Data::Struct(data) = &data else {
        return Err(syn::Error::new_spanned(
            &ident,
            "`CallbackData` can be derived only for `struct`",
        ));
    };
    let Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new_spanned(
            &ident,
            "`CallbackData` can't be derived for structs without named fields",
        ));
    };
    if fields.named.is_empty() {
        return Err(syn::Error::new_spanned(
            &ident,
            "`CallbackData` can't be derived for structs without fields",
        ));
    }
    let attrs = CallbackDataAttrs::parse(&attrs)?.ok_or_else(|| {
        syn::Error::new_spanned(&ident, "missing `#[callback_data(...)]` attribute")
    })?;

    let prefix = &attrs.prefix;
    // `:` by default, check `DEFAULT_SEPARATOR` in `telers` crate
    let separator = attrs.separator.as_ref().map_or(':', LitChar::value);

    let field_idents = fields
        .named
        .iter()
        .map(|field| field.ident.as_ref().expect("named fields are always named"))
        .collect::<Vec<_>>();
    let field_tys = fields.named.iter().map(|field| &field.ty);
    let field_names = field_idents.iter().map(ToString::to_string);
    let field_count = field_idents.len();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let callback_data_impl = quote_spanned! { ident.span() =>
        #[automatically_derived]
        impl #impl_generics ::telers::callback_data::CallbackData for #ident #ty_generics #where_clause {
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
                    ::std::vec![
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
    };

    let self_ty = quote_spanned! { ident.span() => #ident #ty_generics };
    let msg = format!(
        "No found data in context by key `callback_data` or value has wrong type expected \
         `{ident}`. You didn't forget to add the `CallbackData` filter to the handler?"
    );
    let body = quote_spanned! { ident.span() =>
        let res = match request.context.get::<#self_ty>("callback_data") {
            ::std::option::Option::Some(value) => ::std::result::Result::Ok((*value).clone()),
            ::std::option::Option::None => ::std::result::Result::Err(
                ::telers::errors::ExtractionError::new(#msg),
            ),
        };
        async move { res }
    };
    let generics = extractor_generics(
        &generics,
        [parse_quote! {
            #self_ty: ::std::clone::Clone + ::std::marker::Send + 'static
        }],
    );
    let error = quote_spanned! { ident.span() => ::telers::errors::ExtractionError };
    let extractor_impl = tokenize_extractor_impl(ident.span(), &self_ty, &generics, &error, &body);

    Ok(quote_spanned! { ident.span() =>
        #callback_data_impl
        #extractor_impl
    })
}
