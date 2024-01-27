/// This crate contains macros used by `telers` crate
///
/// # Macros
///
/// ## `FromContext`
///
/// Derive an implementation of `FromEventAndContext` for the given type.
/// This macro will generate an implementation of `FromEventAndContext` for the whole given type.
/// It will use the key attribute by which this type will be extracted from context.
pub(crate) mod attrs_parsing;
pub(crate) mod stream;

mod from_context;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::Parse;

/// Derive an implementation of `FromEventAndContext` for the given type.
///
/// # Notes
/// Type must be cloneable because we can't move it out of context, we need to clone it.
///
/// Supported implementations:
/// 1. Implementation of `FromEventAndContext` for the given struct and key attribute by which this type will be extracted from context.
/// 2. Implementation of `FromEventAndContext` for the given enum and key attribute by which this type will be extracted from context.
///
/// # Implementation details
/// This macro will generate an implementation of `FromEventAndContext` for the whole given type.
///
/// ```rust
/// use telers_macros::FromContext;
///
/// #[derive(Clone, FromContext)]
/// #[context(key = "my_struct")]
/// struct MyStruct {
///   field: i32,
/// }
///
/// #[derive(Clone, FromContext)]
/// #[context(key = "my_enum")]
/// enum MyEnum {
///  Variant1,
///  Variant2,
/// }
///
/// async fn handler(my_struct: MyStruct, my_enum: MyEnum) {
///  // ...
/// }
/// ```
#[proc_macro_derive(FromContext, attributes(context))]
pub fn derive_from_context(item: TokenStream) -> TokenStream {
    expand_with(item, from_context::expand)
}

fn expand_with<F, I, K>(input: TokenStream, f: F) -> TokenStream
where
    F: FnOnce(I) -> syn::Result<K>,
    I: Parse,
    K: ToTokens,
{
    expand(syn::parse(input).and_then(f))
}

fn expand<T>(result: syn::Result<T>) -> TokenStream
where
    T: ToTokens,
{
    match result {
        Ok(tokens) => {
            let tokens = (quote! { #tokens }).into();
            if std::env::var_os("MACROS_DEBUG").is_some() {
                eprintln!("{tokens}");
            }
            tokens
        }
        Err(err) => err.into_compile_error().into(),
    }
}
