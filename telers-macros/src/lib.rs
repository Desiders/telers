//! This crate contains the derive macros of the `telers` crate: [`FromContext`], [`FromEvent`], [`CallbackData`] and [`Command`].
//!
//! All of them implement `Extractor`, so the derived types can be used as handler arguments.

mod attrs_parsing;
mod callback_data;
mod command;
mod extractor;
mod from_context;
mod from_event;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

/// Derive an implementation of `Extractor` for the given type.
///
/// This macro supports the following attributes:
/// * `#[context(key = "...")]` - the key by which the type will be extracted from context.
/// * `#[context(into = "...")]` - the type into which the type will be converted.
/// * `#[context(from = "...")]` - the type from which the type will be converted. \
///   `into` and `from` can't be used at the same time.
/// * `#[context(description = "...")]` - the description of the type in context. \
///   This attribute is used only for documentation purposes and perhaps for debugging.
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
///     field: i32,
/// }
///
/// async fn handler(my_struct: MyStruct) {
///     // ...
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
///     Variant1,
///     Variant2,
/// }
///
/// async fn handler(my_enum: MyEnum) {
///     // ...
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
///     field: i32,
/// }
///
/// struct MyStructWrapper(MyStruct);
///
/// impl From<MyStruct> for MyStructWrapper {
///     fn from(my_struct: MyStruct) -> Self {
///         Self(my_struct)
///     }
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
///     field: i32,
/// }
///
/// #[derive(FromContext)]
/// #[context(key = "my_struct", from = MyStruct)]
/// struct MyStructWrapper(MyStruct);
///
/// impl From<MyStruct> for MyStructWrapper {
///     fn from(my_struct: MyStruct) -> Self {
///         Self(my_struct)
///     }
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
///     Variant1,
///     Variant2,
/// }
///
/// struct MyEnumWrapper(MyEnum);
///
/// impl From<MyEnum> for MyEnumWrapper {
///     fn from(my_enum: MyEnum) -> Self {
///         Self(my_enum)
///     }
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
///     Variant1,
///     Variant2,
/// }
///
/// #[derive(FromContext)]
/// #[context(key = "my_enum", from = MyEnum)]
/// struct MyEnumWrapper(MyEnum);
///
/// impl From<MyEnum> for MyEnumWrapper {
///     fn from(my_enum: MyEnum) -> Self {
///         Self(my_enum)
///     }
/// }
/// ```
#[proc_macro_derive(FromContext, attributes(context))]
pub fn derive_from_context(item: TokenStream) -> TokenStream {
    expand_with(item, from_context::expand)
}

/// Derive an implementation of `Extractor` for the given type.
///
/// This macro supports the following attributes:
/// * `#[event(from = "...")]` - the from which the type will be converted.
/// * `#[event(try_from = "...")]` - the from which the type will be converted.
/// * `#[event(error = "...")]` - the error type that will be returned if conversion fails. \
///   Used only if `try_from` is specified. \
///   If it's empty, then we use `ConvertToTypeError` type as error type. \
///   If it's not empty, then we use this type as error type.
/// * `#[event(description = "...")]` - the description of the type. \
///   This attribute is used only for documentation purposes.
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
/// use telers::types::Update;
/// use telers_macros::FromEvent;
///
/// #[derive(FromEvent)]
/// #[event(from = Update)]
/// struct UpdateId(i64);
///
/// impl From<Update> for UpdateId {
///     fn from(update: Update) -> Self {
///         Self(update.update_id())
///     }
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
///     type Error = Infallible;
///
///     fn try_from(update: Update) -> Result<Self, Self::Error> {
///         Ok(Self(update.update_id()))
///     }
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
///     type Error = ConvertToTypeError;
///
///     fn try_from(update: Update) -> Result<Self, Self::Error> {
///         match update.from().map(|user| user.id) {
///             Some(id) => Ok(Self(id)),
///             None => Err(ConvertToTypeError::new("Update", "UpdateFromId")),
///         }
///     }
/// }
/// ```
/// # Notes
/// This macros is used in the library to implement `Extractor` for types that impl `From` for `Update`,
/// but you can use it for your own types.
#[proc_macro_derive(FromEvent, attributes(event))]
pub fn derive_from_event(item: TokenStream) -> TokenStream {
    expand_with(item, from_event::expand)
}

/// Derive an implementation of `CallbackData` for the given struct.
///
/// This macro generates:
/// * `CallbackData` implementation with `pack` and `unpack` methods.
///   Fields must implement `CallbackDataValue` (implemented for primitives, `String`, `Box<str>` and `Option<T>`).
/// * `Extractor` implementation to extract the unpacked data from context
///   (the `CallbackData` filter from `telers::filters` places it there).
///
/// This macro supports the following attributes:
/// * `#[callback_data(prefix = "...")]` - the prefix of callback data (required).
///   It identifies the callback data type, so it should be unique for each type.
/// * `#[callback_data(separator = '...')]` - the separator of callback data values (optional, `:` by default).
///
/// # Example
///
/// ```rust
/// use telers::{
///     filters::CallbackData as CallbackDataFilter, utils::callback_data::CallbackData as _,
///     CallbackData,
/// };
///
/// #[derive(CallbackData, Clone)]
/// #[callback_data(prefix = "language")]
/// struct LanguageSettings {
///     language_code: String,
///     enabled: bool,
/// }
///
/// // Packing data to a string and sending it with a button
/// let callback_data = LanguageSettings {
///     language_code: "en".into(),
///     enabled: true,
/// }
/// .pack()
/// .unwrap();
/// assert_eq!(callback_data, "language:en:1");
///
/// // Unpacking data from a callback query string
/// let unpacked = LanguageSettings::unpack("language:en:1").unwrap();
/// assert_eq!(unpacked.language_code, "en");
/// assert!(unpacked.enabled);
///
/// // Filtering callback queries and extracting data in handlers
/// let router: telers::Router =
///     telers::Router::new("language settings").on_callback_query(|observer| {
///         observer.filter(CallbackDataFilter::<LanguageSettings>::new())
///     });
/// ```
#[proc_macro_derive(CallbackData, attributes(callback_data))]
pub fn derive_callback_data(item: TokenStream) -> TokenStream {
    expand_with(item, callback_data::expand)
}

/// Derive an implementation of `Extractor` for the given enum.
///
/// The macro generates an `Extractor` implementation that parses the command name and arguments
/// from the [`CommandObject`] that the [`Command`] filter puts into the context, so the derived
/// enum can be used as a handler argument right after the filter.
///
/// Command names are derived from the variant names and are matched case-insensitively.
/// Fields of tuple and named variants are parsed from the command arguments in declaration order
/// via the [`CommandArg`] trait; a missing or extra argument or a parse failure is reported
/// as an [`ExtractionError`].
///
/// This macro supports the following attributes:
/// * `#[command(rename_rule = "...")]` (enum-level) - the rule used to convert variant names into command names. \
///   Supported rules: `lowercase` (default), `snake_case`.
/// * `#[command(prefix = '!')]` (enum-level) - the command prefix, `/` by default. \
///   It is a part of the command, so `!start` and `/start` are different commands.
/// * `#[command(split = ',')]` (enum-level) - the character the command arguments are split on. \
///   By default (or with `' '`) they are split on any run of whitespace.
/// * `#[command(description = "...")]` (variant-level, optional) - the description of the command. \
///   Used by the generated `descriptions()` and `bot_commands()` methods.
/// * `#[command(hidden)]` (variant-level, optional) - excludes the variant from `descriptions()` \
///   and `bot_commands()`, but the command stays matchable.
/// * `#[command(aliases = ["a", "b"])]` (variant-level, optional) - extra names the variant matches.
/// * `#[command(rename = "custom_name")]` (variant-level, optional) - an explicit command name \
///   overriding the `rename_rule` for that variant.
/// * `#[command(prefix = '!')]` (variant-level) - a per-variant override of the enum-level value.
/// * `#[command(split = ',')]` (variant-level) - a per-variant override of the enum-level value.
///
/// Besides the `Extractor` implementation, the macro generates:
/// * `descriptions()` - descriptions in the format `/command - description` (with the prefix of the command), \
///   separated by newlines.
/// * `bot_commands()` - commands in the format required by the `setMyCommands` Telegram API method. \
///   Only commands with the `/` prefix are included, because the method supports no other prefix.
///
/// # Notes
/// * The [`Command`] filter must be used together with the derived enum,
///   because the macro reads the [`CommandObject`] from the context.
/// * Extra arguments are an error: use `Option<T>`, `Vec<T>` or `Rest` fields to take them.
///
/// # Example
/// ```rust
/// use telers_macros::Command;
///
/// #[derive(Command)]
/// #[command(rename_rule = "snake_case")]
/// enum Commands {
///     #[command(description = "display this text")]
///     Help,
///     #[command(description = "handle a username")]
///     Username(String),
///     #[command(description = "handle a username and an age")]
///     UsernameAndAge { username: String, age: u8 },
/// }
///
/// async fn handler(commands: Commands) {
///     match commands {
///         Commands::Help => {}
///         Commands::Username(username) => {}
///         Commands::UsernameAndAge {
///             username,
///             age,
///         } => {}
///     }
/// }
/// ```
///
/// [`CommandObject`]: telers::filters::CommandObject
/// [`Command`]: telers::filters::Command
/// [`CommandArg`]: telers::utils::command_args::CommandArg
/// [`ExtractionError`]: telers::errors::ExtractionError
#[proc_macro_derive(Command, attributes(command))]
pub fn derive_command(item: TokenStream) -> TokenStream {
    expand_with(item, command::expand)
}

/// Parses the input of the derive macro and expands it with `f`,
/// the error of any of the steps is emitted as a compile error
fn expand_with<F, T>(input: TokenStream, f: F) -> TokenStream
where
    F: FnOnce(DeriveInput) -> syn::Result<T>,
    T: ToTokens,
{
    match syn::parse(input).and_then(f) {
        Ok(tokens) => {
            let tokens = tokens.into_token_stream().into();
            // Prints the generated code for debugging of the macros
            if std::env::var_os("MACROS_DEBUG").is_some() {
                eprintln!("{tokens}");
            }
            tokens
        }
        Err(err) => err.into_compile_error().into(),
    }
}
