//! This module contains the generation of the `Extractor` impls shared by the macros

use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use syn::{parse_quote, Generics, WherePredicate};

/// Generics of the `Extractor` impl for the type
/// # Arguments
/// * `generics` - Generics of the type
/// * `predicates` - Bounds to add to the where clause of the impl
/// # Returns
/// The generics of the type with the client generic parameter `__C` of `Extractor` added
/// (prefixed with `__` to avoid conflicts with the generics of the type) and `predicates` in the where clause
#[must_use]
pub(crate) fn extractor_generics(
    generics: &Generics,
    predicates: impl IntoIterator<Item = WherePredicate>,
) -> Generics {
    let mut generics = generics.clone();
    generics.params.push(parse_quote! { __C });
    generics.make_where_clause().predicates.extend(predicates);
    generics
}

/// `Extractor<__C>` impl for the type
/// # Arguments
/// * `span` - Span of the type for the errors in the generated code
/// * `self_ty` - Type for which the trait is implemented, with its generics
/// * `generics` - Generics of the impl (see [`extractor_generics`])
/// * `error` - Error type of the extraction
/// * `body` - Body of the `extract` method
#[must_use]
pub(crate) fn tokenize_extractor_impl(
    span: Span,
    self_ty: &TokenStream,
    generics: &Generics,
    error: &TokenStream,
    body: &TokenStream,
) -> TokenStream {
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    quote_spanned! { span =>
        #[automatically_derived]
        impl #impl_generics ::telers::Extractor<__C> for #self_ty #where_clause {
            type Error = #error;

            #[inline]
            fn extract(
                request: &::telers::Request<__C>,
            ) -> impl ::std::future::Future<Output = ::std::result::Result<Self, Self::Error>> + ::std::marker::Send {
                #body
            }
        }
    }
}
