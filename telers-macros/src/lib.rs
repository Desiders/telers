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
mod from_event;

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
/// #[derive(FromContext)]
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
/// #[derive(FromContext)]
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

/// Derive an implementation of `FromEventAndContext` for the given type.
///
/// This macro supports the following attributes:
/// * `#[event(from = "...")]` - the from which the type will be converted.
/// * `#[event(try_from = "...")]` - the from which the type will be converted.
/// * `#[event(error = "...")]` - the error type that will be returned if conversion fails. \
/// Used only if `try_from` is specified. \
/// If it's empty, then we use `ConvertToTypeError` type as error type. \
/// If it's not empty, then we use this type as error type.
/// * `#[event(description = "...")]` - the description of the type. \s
/// This attribute is used only for documentation purposes.
///
/// "..." it can be either a type, or a type path to one of them:
/// * `Update` - the main type of the crate, which contains all the information about the event.
///
/// Check examples below to see how to use this macro and what types of deriving are supported.
///
/// ## Whole struct that can be converted from `Update`
///
/// You need to implement `From`/`TryFrom` trait for your type by yourself.
/// This can be useful when you want to use some type from the `Update` in your handler in a more convenient way.
///
/// ```rust
/// use telers_macros::FromEvent;
/// use telers::types::Update;
///
/// #[derive(FromEvent)]
/// #[event(from = Update)]
/// struct UpdateId(i64);
///
/// impl From<Update> for UpdateId {
///  fn from(update: Update) -> Self {
///   Self(update.id)
///  }
/// }
/// ```
///
/// You can also use `#[event(try_from = "...")]` attribute to specify the type from which the type will be converted.
///
/// ```rust
/// use telers_macros::FromEvent;
/// use telers::types::Update;
/// use std::convert::Infallible;
///
/// #[derive(FromEvent)]
/// #[event(try_from = Update, error = Infallible)] // we can don't specify error type, but it will be `ConvertToTypeError` by default
/// struct UpdateId(i64);
///
/// impl TryFrom<Update> for UpdateId { // we use `TryFrom` here just for example, you need to use `From` if error is impossible
///  type Error = Infallible;
///
///  fn try_from(update: Update) -> Result<Self, Self::Error> {
///   Ok(Self(update.id))
///  }
/// }
/// ```
///
/// Another example, but with default error type:
///
/// ```rust
/// use telers_macros::FromEvent;
/// use telers::{types::Update, errors::ConvertToTypeError};
/// use std::convert::Infallible;
///
/// #[derive(FromEvent)]
/// #[event(try_from = Update)] // you can specify `ConvertToTypeError` as error type, but it's not necessary, because it's default
/// struct UpdateFromId(i64);
///
/// impl TryFrom<Update> for UpdateFromId {
///  type Error = ConvertToTypeError;
///
///  fn try_from(update: Update) -> Result<Self, Self::Error> {
///   match update.from_id() {
///    Some(id) => Ok(Self(id)),
///    None => Err(ConvertToTypeError::new("Update", "UpdateFromId")),
///   }
///  }
/// }
/// ```
/// # Notes
/// This macros is used in the library to implement `FromEventAndContext` for types that impl `From` for `Update`,
/// but you can use it for your own types.
#[proc_macro_derive(FromEvent, attributes(event))]
pub fn derive_from_event(item: TokenStream) -> TokenStream {
    expand_with(item, from_event::expand)
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
