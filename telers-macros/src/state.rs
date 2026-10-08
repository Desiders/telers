//! This module contains the expansion of the `State` derive

use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{
    punctuated::Punctuated, spanned::Spanned, Data, DeriveInput, Fields, Ident, Token, Variant,
};

/// Collects the state name of every variant, `AwaitingPayment` -> `awaiting_payment`
fn expand_names(variants: &Punctuated<Variant, Token![,]>) -> syn::Result<Vec<(&Ident, String)>> {
    let mut names = Vec::with_capacity(variants.len());
    for variant in variants {
        if !matches!(variant.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(
                variant,
                "only unit variants are supported",
            ));
        }

        let name = variant.ident.to_string().to_snake_case();
        if names.iter().any(|(_, seen)| seen == &name) {
            return Err(syn::Error::new_spanned(
                &variant.ident,
                format!("duplicate state name `{name}`"),
            ));
        }

        names.push((&variant.ident, name));
    }
    Ok(names)
}

pub(crate) fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = input;

    if !generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            generics,
            "generic states are not supported",
        ));
    }

    let Data::Enum(data) = data else {
        return Err(syn::Error::new_spanned(
            &ident,
            "expected `enum` with `State` derive",
        ));
    };

    let names = expand_names(&data.variants)?;
    let arms = names.iter().map(|(variant, name)| {
        quote_spanned! { variant.span() => Self::#variant => #name }
    });

    Ok(quote_spanned! { ident.span() =>
        #[allow(dead_code)]
        impl #ident {
            /// Returns the name of the state as it is stored in the storage
            #[must_use]
            pub const fn as_str(&self) -> &'static str {
                match self {
                    #(#arms,)*
                }
            }
        }

        #[automatically_derived]
        impl ::std::convert::AsRef<str> for #ident {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        #[automatically_derived]
        impl ::std::cmp::PartialEq<&str> for #ident {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }

        #[automatically_derived]
        impl ::std::fmt::Display for #ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    })
}
