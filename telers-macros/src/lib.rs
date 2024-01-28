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
/// This macro supports the following attributes:
/// * `#[context(key = "...")]` - the key by which the type will be extracted from context.
/// * `#[context(into = "...")]` - the type into which the type will be converted.
/// * `#[context(from = "...")]` - the type from which the type will be converted.
/// * `#[context(description = "...")]` - the description of the type in context. \
/// This attribute is used only for documentation purposes and perhaps for debugging.
///
/// Check the examples below to see how to use this macro and what types of deriving are supported.
///
/// ## Whole struct by key in context
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
/// async fn handler(my_struct: MyStruct) {
///  // ...
/// }
/// ```
///
/// ## Whole enum by key in context
///
/// ```rust
/// use telers_macros::FromContext;
///
/// #[derive(Clone, FromContext)]
/// #[context(key = "my_enum")]
/// enum MyEnum {
///  Variant1,
///  Variant2,
/// }
///
/// async fn handler(my_enum: MyEnum) {
///  // ...
/// }
/// ```
///
/// ## Whole struct that can be converted from another one type that is in context by key
///
/// You need to implement `From`/`Into` trait for your type by yourself.
/// This can be useful when you want to wrap your type to another one or if the type in context is a foreign type,
/// and you want to convert it to your own type to use it in handler (because you can't implement a foreign trait for a foreign type).
///
/// ```rust
/// use telers_macros::FromContext;
///
/// #[derive(Clone, FromContext)]
/// #[context(key = "my_struct", into = MyStructWrapper)]
/// struct MyStruct {
///  field: i32,
/// }
///
/// struct MyStructWrapper(MyStruct);
///
/// impl From<MyStruct> for MyStructWrapper {
///  fn from(my_struct: MyStruct) -> Self {
///   Self(my_struct)
///  }
/// }
/// ```
///
/// You can also use `#[context(from = "...")]` attribute to specify the type from which the type will be converted.
///
/// ```rust
/// use telers_macros::FromContext;
///
/// #[derive(Clone)]
/// struct MyStruct {
///  field: i32,
/// }
///
/// #[derive(Clone, FromContext)]
/// #[context(key = "my_struct", from = MyStruct)]
/// struct MyStructWrapper(MyStruct);
///
/// impl From<MyStruct> for MyStructWrapper {
///  fn from(my_struct: MyStruct) -> Self {
///   Self(my_struct)
///  }
/// }
/// ```
///
/// ## Whole enum that can be converted from another one type that is in context by key
///
/// You need to implement `From`/`Into` trait for your type by yourself.
/// This can be useful when you want to wrap your type to another one or if the type in context is a foreign type,
/// and you want to convert it to your own type to use it in handler (because you can't implement a foreign trait for a foreign type).
///
/// ```rust
/// use telers_macros::FromContext;
///
/// #[derive(Clone, FromContext)]
/// #[context(key = "my_enum", into = MyEnumWrapper)]
/// enum MyEnum {
///  Variant1,
///  Variant2,
/// }
///
/// struct MyEnumWrapper(MyEnum);
///
/// impl From<MyEnum> for MyEnumWrapper {
///  fn from(my_enum: MyEnum) -> Self {
///   Self(my_enum)
///  }
/// }
/// ```
///
/// You can also use `#[context(from = "...")]` attribute to specify the type from which the type will be converted.
///
/// ```rust
/// use telers_macros::FromContext;
///
/// #[derive(Clone)]
/// enum MyEnum {
///  Variant1,
///  Variant2,
/// }
///
/// #[derive(Clone, FromContext)]
/// #[context(key = "my_enum", from = MyEnum)]
/// struct MyEnumWrapper(MyEnum);
///
/// impl From<MyEnum> for MyEnumWrapper {
///  fn from(my_enum: MyEnum) -> Self {
///   Self(my_enum)
///  }
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
