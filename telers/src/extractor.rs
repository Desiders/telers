//! This module contains functionality for extracting data to the handler arguments.
//!
//! [`Extractor`] is the main trait which need to be implemented for extracting data.
//! If you want to use your own types as handler arguments, you need to implement this trait for them.
//! By default, this trait is implemented for the most common middlewares, types and filters, so you can use them without any additional actions.
//! The trait also is implemented for `Option<T>`, `Result<T, E>` where `T: Extractor`,
//! so you can don't implement it for your types if you want to use them as optional or result arguments.
//!
//! # Using extensions
//!
//! You can use [`Extension`] to extract data from [`Extensions`] that can be easily filled in,
//! for example in middlewares:
//! ```rust
//! use telers::{
//!     errors::EventErrorKind,
//!     event::{telegram::HandlerResult, EventReturn},
//!     middlewares::outer::{Middleware, MiddlewareResponse},
//!     Extension, Request,
//! };
//!
//! #[derive(Clone)]
//! struct ToExtensionsMiddleware<T> {
//!     data: T,
//! }
//!
//! impl<T> Middleware for ToExtensionsMiddleware<T>
//! where
//!     T: Send + Sync + Clone + 'static,
//! {
//!     async fn call(
//!         &mut self,
//!         mut request: Request,
//!     ) -> Result<MiddlewareResponse, EventErrorKind> {
//!         request.extensions.insert(self.data.clone());
//!
//!         Ok((request, EventReturn::default()))
//!     }
//! }
//!
//! async fn send_data_handler<T>(Extension(data2): Extension<T>) -> HandlerResult {
//!     todo!();
//! }
//! ```
//!
//! You can check examples of usage extensions in the [`examples`] directory.
//!
//! # Implementing trait
//!
//! Ways to implement [`Extractor`] for your own types:
//! * Implement it directly (much boilerplate code, but it's needed for complex types)
//! * Use the [`FromContext`] macro (simple way to implement this for types in a [`Context`] by its key)
//! * Use the [`FromEvent`] macro (simple way to implement this for types in an event, for example, [`crate::types::Update`])
//!
//! ## Implementing directly
//!
//! Simple example with extracting id from [`crate::types::Update`]:
//!
//! ```rust
//! use std::convert::Infallible;
//! use telers::{Extractor, Request};
//!
//! struct UpdateId(i64);
//!
//! impl Extractor for UpdateId {
//!     type Error = Infallible;
//!
//!     async fn extract(request: &Request) -> Result<Self, Self::Error> {
//!         Ok(UpdateId(request.update.update_id()))
//!     }
//! }
//! ```
//!
//! This example will extract the [`crate::types::Update`] id to the handler argument.
//! After that, you can use this argument in the handler:
//!
//! ```ignore
//! async fn handler(update_id: UpdateId) {
//!     println!("Update id: {}", id.0);
//! }
//! ```
//!
//! Another example with extracting id of the user who sent the message from [`crate::types::Update`]:
//!
//! ```rust
//! use telers::{errors::ConvertToTypeError, Extractor, Request};
//!
//! struct UpdateFromId(i64);
//!
//! impl Extractor for UpdateFromId {
//!     type Error = ConvertToTypeError;
//!
//!     // you can use your own error type, this is just an example
//!
//!     async fn extract(request: &Request) -> Result<Self, Self::Error> {
//!         match request.update.from() {
//!             Some(from) => Ok(UpdateFromId(from.id)),
//!             None => Err(ConvertToTypeError::new("Update", "UpdateFromId")),
//!         }
//!     }
//! }
//! ```
//!
//! In some cases we sure that some data is not none, so in one handler we can use `Option` and in another handler we can use the type directly.
//! After we implemented [`Extractor`] for our type, we can use it in both cases,
//! because the trait is implemented for `Option<T>` and `Result<T, E>` where `T: Extractor`:
//!
//! ```rust
//! use telers::{errors::ConvertToTypeError, Extractor, Request};
//!
//! struct UpdateFromId(i64);
//!
//! impl Extractor for UpdateFromId {
//!     type Error = ConvertToTypeError;
//!
//!     // you can use your own error type, this is just an example
//!
//!     async fn extract(request: &Request) -> Result<Self, Self::Error> {
//!         match request.update.from() {
//!             Some(from) => Ok(UpdateFromId(from.id)),
//!             None => Err(ConvertToTypeError::new("Update", "UpdateFromId")),
//!         }
//!     }
//! }
//! ```
//!
//! After that, you can use this argument in the handlers:
//!
//! ```ignore
//! // Here `from_id` can't be `None` (for example we use filter which checks that `from_id` is not `None`)
//! async fn handler_first(from_id: UpdateFromId) {
//!     println!("Update from id: {}", from_id.0);
//! }
//!
//! // Here `from_id` can be `None`
//! async fn handler_second(from_id: Option<UpdateFromId>) {
//!     if let Some(from_id) = from_id {
//!         println!("Update from id: {}", from_id.0);
//!     }
//! }
//! ```
//!
//! ## Implementing with [`FromEvent`] macro
//!
//! Simple example with extracting id from [`crate::types::Update`]:
//!
//! ```rust
//! use telers::{types::Update, FromEvent};
//!
//! #[derive(FromEvent)]
//! #[event(from = Update)]
//! struct UpdateId(i64);
//!
//! // We need to implement `From<Update>` for `UpdateId` by ourselves (this is required by `FromEvent` macro)
//! impl From<Update> for UpdateId {
//!     fn from(update: Update) -> Self {
//!         Self(update.update_id())
//!     }
//! }
//! ```
//!
//! Here we used `#[event(from = Update)]` attribute to specify the type from which the type will be converted.
//!
//! We also can use `#[event(try_from = "...")]`, but in this case we need to implement [`TryFrom`] for our type instead of [`From`]:
//!
//! ```rust
//! use telers::{types::Update, FromEvent, errors::ConvertToTypeError};
//!
//! #[derive(FromEvent)]
//! #[event(try_from = Update)] // you can specify [`ConvertToTypeError`] as error type, but it's not necessary, because it's default
//! struct UpdateFromId(i64);
//!
//! impl TryFrom<Update> for UpdateFromId {
//!     type Error = ConvertToTypeError;
//!
//!     fn try_from(update: Update) -> Result<Self, Self::Error> {
//!         match update.from() {
//!             Some(from) => Ok(Self(from.id)),
//!             None => Err(ConvertToTypeError::new("Update", "UpdateFromId")),
//!         }
//!     }
//! }
//! ```
//!
//! By default, the error type is [`ConvertToTypeError`](telers::errors::ConvertToTypeError),
//! but you can specify your own error type with `#[event(error = "...")]` attribute:
//!
//! ```rust
//! use std::convert::Infallible;
//! use telers::{types::Update, FromEvent};
//!
//! #[derive(FromEvent)]
//! #[event(try_from = Update, error = Infallible)]
//! struct UpdateId(i64);
//!
//! impl TryFrom<Update> for UpdateId {
//!     // we use `TryFrom` here just for example, you need to use `From` if error is impossible
//!     type Error = Infallible;
//!
//!     fn try_from(update: Update) -> Result<Self, Self::Error> {
//!         Ok(Self(update.update_id()))
//!     }
//! }
//! ```
//!
//! ## Implementing with [`FromContext`] macro
//!
//! Simple example with extracting struct by key from [`Context`]:
//!
//! ```rust
//! use telers_macros::FromContext;
//!
//! #[derive(Clone, FromContext)]
//! #[context(key = "my_struct")]
//! struct MyStruct {
//!     field: i32,
//! }
//! ```
//!
//! Now we can use `MyStruct` as handler argument if we put it in the context with key `my_struct`.
//! There is a serious problem here: we don't know where struct by key `my_struct` is set to the context
//! and if context doesn't contain type by key `my_struct` we need to know where the source of the problem is.
//! We can use `#[content(description = "...")]` to describe where the structure is installed, or cases where it is not installed, for example:
//!
//! ```rust
//! use telers_macros::FromContext;
//!
//! #[derive(Clone, FromContext)]
//! #[context(
//!     key = "my_struct",
//!     description = "This struct is set in the `MyMiddleware` middleware. If it is not set, \
//!                    then the `MyMiddleware` middleware is not used."
//! )]
//! struct MyStruct {
//!     field: i32,
//! }
//! ```
//!
//! In some cases, you may want to use a one type in context, but extract it as another type.
//! For this case, you can use `#[context(into = "...")]` attribute:
//!
//! ```rust
//! use telers_macros::FromContext;
//!
//! #[derive(Clone, FromContext)]
//! #[context(key = "my_struct", into = MyStructWrapper)]
//! struct MyStruct {
//!     field: i32,
//! }
//!
//! struct MyStructWrapper(MyStruct);
//!
//! impl From<MyStruct> for MyStructWrapper {
//!     fn from(my_struct: MyStruct) -> Self {
//!         Self(my_struct)
//!     }
//! }
//! ```
//!
//! This code will extract `MyStruct` from context and convert it to `MyStructWrapper`,
//! but we need to implement `From<MyStruct>` for `MyStructWrapper` by ourselves (this is required by [`FromContext`] macro).
//! In this case, the trait is implements for `MyStructWrapper`, not for `MyStruct`,
//! so we can't use `MyStruct` as handler argument without implementing `Extractor` for it.
//!
//! We also can use `#[context(from = "...")]` attribute to specify the type from which the type will be converted:
//!
//! ```rust
//! use telers_macros::FromContext;
//!
//! #[derive(Clone)]
//! struct MyStruct {
//!     field: i32,
//! }
//!
//! #[derive(FromContext)]
//! #[context(key = "my_struct", from = MyStruct)]
//! struct MyStructWrapper(MyStruct);
//!
//! impl From<MyStruct> for MyStructWrapper {
//!     fn from(my_struct: MyStruct) -> Self {
//!         Self(my_struct)
//!     }
//! }
//! ```
//!
//! This code similar to the previous one, but more useful in cases when `from` type is a foreign type.
//!
//! [`FromEvent`]: telers::FromEvent
//! [`FromContext`]: telers::FromContext
//! [`Extensions`]: telers::extensions::Extensions
//! [`examples`]: https://github.com/Desiders/telers/tree/dev-1.x/examples

use crate::{
    client::{Bot, Reqwest},
    context::Context,
    either::Either,
    errors::ExtractionError,
    extensions::Extension,
    Extensions, Request,
};

use std::{any::type_name, convert::Infallible, future::Future};

/// Trait for extracting data from [`crate::types::Update`] and [`Context`] to handlers arguments
pub trait Extractor<Client = Reqwest>: Sized {
    type Error: Into<ExtractionError>;

    /// Extracts data to handler argument
    /// # Errors
    /// If extraction was unsuccessful
    ///
    /// Possible variants:
    /// * No found data in context by key
    /// * Data in context by key has wrong type. For example, you try to extract `i32` from `String`.
    /// * Custom user error
    fn extract(request: &Request<Client>)
        -> impl Future<Output = Result<Self, Self::Error>> + Send;
}

/// To be able to use [`Option`] as handler argument
/// This implementation will return `None` if extraction was unsuccessful, and `Some(value)` otherwise
impl<Client, T: Extractor<Client>> Extractor<Client> for Option<T>
where
    Client: Sync,
{
    type Error = Infallible;

    #[inline]
    async fn extract(request: &Request<Client>) -> Result<Self, Self::Error> {
        match T::extract(request).await {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None),
        }
    }
}

/// To be able to use [`Result`] as handler argument
/// This implementation will return `Ok(value)` if extraction was successful, and `Err(error)` otherwise,
/// where `error` is `T::Error` converted to `E`
impl<Client, T, E> Extractor<Client> for Result<T, E>
where
    T: Extractor<Client>,
    T::Error: Into<E>,
    Client: Sync,
{
    type Error = Infallible;

    #[inline]
    async fn extract(request: &Request<Client>) -> Result<Self, Self::Error> {
        Ok(T::extract(request).await.map_err(Into::into))
    }
}

/// To be able to use [`Either`] as handler argument
/// Extraction is attempted left-to-right: if `T` extracts successfully, [`Either::Left`] is
/// returned; otherwise `U` is tried and [`Either::Right`] is returned on success. If both
/// fail, the error from the last attempt (`U`) is returned.
impl<Client, T, U> Extractor<Client> for Either<T, U>
where
    T: Extractor<Client>,
    U: Extractor<Client>,
    Client: Sync,
{
    type Error = U::Error;

    #[inline]
    async fn extract(request: &Request<Client>) -> Result<Self, Self::Error> {
        if let Ok(value) = T::extract(request).await {
            return Ok(Either::Left(value));
        }
        U::extract(request).await.map(Either::Right)
    }
}

/// To be able to use handler without arguments
/// Handler without arguments will be called with `()` argument (unit type)
impl<Client> Extractor<Client> for () {
    type Error = Infallible;

    #[allow(clippy::manual_async_fn)]
    #[inline]
    fn extract(
        _request: &Request<Client>,
    ) -> impl Future<Output = Result<Self, Self::Error>> + Send {
        async move { Ok(()) }
    }
}

impl<Client> Extractor<Client> for Bot<Client>
where
    Client: Clone + Send,
{
    type Error = Infallible;

    #[inline]
    fn extract(
        request: &Request<Client>,
    ) -> impl Future<Output = Result<Self, Self::Error>> + Send {
        let bot = request.bot.clone();
        async move { Ok(bot) }
    }
}

impl<Client> Extractor<Client> for Context {
    type Error = Infallible;

    #[inline]
    fn extract(
        request: &Request<Client>,
    ) -> impl Future<Output = Result<Self, Self::Error>> + Send {
        let context = request.context.clone();
        async move { Ok(context) }
    }
}

impl<Client> Extractor<Client> for Extensions {
    type Error = Infallible;

    #[inline]
    fn extract(
        request: &Request<Client>,
    ) -> impl Future<Output = Result<Self, Self::Error>> + Send {
        let extensions = request.extensions.clone();
        async move { Ok(extensions) }
    }
}

impl<Client, Value> Extractor<Client> for Extension<Value>
where
    Value: Clone + Send + Sync + 'static,
{
    type Error = ExtractionError;

    fn extract(
        request: &Request<Client>,
    ) -> impl Future<Output = Result<Self, Self::Error>> + Send {
        let res = match request.extensions.get::<Value>() {
            Some(value) => Ok(Self(value.clone())),
            None => Err(ExtractionError::new(if request.extensions.is_empty() {
                format!(
                    "Failed to extract data with type {}. Extensions are empty, it looks like you \
                     forgot to add a value.",
                    type_name::<Value>()
                )
            } else {
                format!(
                    "Failed to extract data with type {}. It looks like you forgot to add a value \
                     of this type.",
                    type_name::<Value>()
                )
            })),
        };
        async move { res }
    }
}

#[allow(non_snake_case)]
mod factory_extractor {
    //! This module is used to implement [`Extractor`] for tuple arguments, each of which implements it
    //! If one of the arguments fails to extract, the whole extraction fails, and the error is returned

    use super::{ExtractionError, Extractor, Request};

    macro_rules! factory ({ $($param:ident)* } => {
        impl<Client: Sync, $($param: Extractor<Client> + Send,)*> Extractor<Client> for ($($param,)*) {
            type Error = ExtractionError;

            async fn extract(request: &Request<Client>) -> Result<Self, Self::Error> {
                Ok(($($param::extract(request).await.map_err(Into::into)?,)*))
            }
        }
    });

    // To be able to extract tuple with 1 arguments
    factory! { A }
    // To be able to extract tuple with 2 arguments
    factory! { A B }
    // To be able to extract tuple with 3 arguments
    factory! { A B C }
    // To be able to extract tuple with 4 arguments
    factory! { A B C D }
    // To be able to extract tuple with 5 arguments
    factory! { A B C D E}
    // To be able to extract tuple with 6 arguments
    factory! { A B C D E F }
    // To be able to extract tuple with 7 arguments
    factory! { A B C D E F G}
    // To be able to extract tuple with 8 arguments
    factory! { A B C D E F G H }
    // To be able to extract tuple with 9 arguments
    factory! { A B C D E F G H I}
    // To be able to extract tuple with 10 arguments
    factory! { A B C D E F G H I J }
    // To be able to extract tuple with 11 arguments
    factory! { A B C D E F G H I J K}
    // To be able to extract tuple with 12 arguments
    factory! { A B C D E F G H I J K L }
    // To be able to extract tuple with 13 arguments
    factory! { A B C D E F G H I J K L M}
    // To be able to extract tuple with 14 arguments
    factory! { A B C D E F G H I J K L M N }
    // To be able to extract tuple with 15 arguments
    factory! { A B C D E F G H I J K L M N O}
    // To be able to extract tuple with 16 arguments
    factory! { A B C D E F G H I J K L M N O P }
}

#[allow(unreachable_code, clippy::extra_unused_type_parameters)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        errors::ConvertToTypeError,
        types::{Message, MessageText, Update},
    };

    use std::sync::Arc;

    #[test]
    fn test_arg_number() {
        fn assert_impl_handler<Client, T: Extractor<Client>>(_: T) {}

        assert_impl_handler::<Reqwest, _>(());
        assert_impl_handler::<Reqwest, _>((
            (), // 1
            (), // 2
            (), // 3
            (), // 4
            (), // 5
            (), // 6
            (), // 7
            (), // 8
            (), // 9
            (), // 10
            (), // 11
            (), // 12
            (), // 13
            (), // 14
            (), // 15
            (), // 16
        ));
    }

    fn _check_bounds<Client, T: Extractor<Client>>() {
        unimplemented!("This function is only used for checking bounds");

        _check_bounds::<Client, ()>();

        _check_bounds::<_, Bot>();
        _check_bounds::<Client, Update>();
        _check_bounds::<Client, Arc<Update>>();
        _check_bounds::<Client, Context>();
        _check_bounds::<Client, Extensions>();

        _check_bounds::<Client, Message>();
        _check_bounds::<Client, MessageText>();
    }

    fn _check_bounds_option<Client: Sync, T: Extractor<Client>>() {
        unimplemented!("This function is only used for checking bounds");

        _check_bounds::<Client, Option<()>>();

        _check_bounds::<_, Option<Bot>>();
        _check_bounds::<Client, Option<Update>>();
        _check_bounds::<Client, Option<Arc<Update>>>();
        _check_bounds::<Client, Option<Context>>();
        _check_bounds::<Client, Option<Extensions>>();

        _check_bounds::<Client, Option<Message>>();
        _check_bounds::<Client, Option<MessageText>>();
    }

    fn _check_bounds_result<Client: Sync, T: Extractor<Client>, Err: Into<ExtractionError>>() {
        unimplemented!("This function is only used for checking bounds");

        _check_bounds::<Client, Result<(), Infallible>>();

        _check_bounds::<_, Result<Bot, Infallible>>();
        _check_bounds::<Client, Result<Update, Infallible>>();
        _check_bounds::<Client, Result<Arc<Update>, Infallible>>();
        _check_bounds::<Client, Result<Context, Infallible>>();
        _check_bounds::<Client, Result<Extensions, Infallible>>();

        _check_bounds::<Client, Result<Message, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageText, ConvertToTypeError>>();
    }

    fn _check_bounds_either<Client: Sync>() {
        unimplemented!("This function is only used for checking bounds");

        _check_bounds::<Client, Either<(), ()>>();

        _check_bounds::<_, Either<Bot, Bot>>();
        _check_bounds::<Client, Either<Update, Context>>();
        _check_bounds::<Client, Either<Message, MessageText>>();
        _check_bounds::<_, Either<Extension<i32>, Bot>>();
        _check_bounds::<Client, Either<Option<Message>, ()>>();
    }

    #[tokio::test]
    async fn extract_either_prefers_left_then_right() {
        use crate::types::{ChatPrivate, UpdateMessage};

        let request = Request::<Reqwest> {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(0), ""),
            ))),
            bot: Bot::default(),
            context: Context::default(),
            extensions: Extensions::default(),
        };

        // `Bot` always extracts, so the left side wins.
        let left = <Either<Bot, Extension<i32>> as Extractor>::extract(&request)
            .await
            .expect("left side is infallible");
        assert!(matches!(left, Either::Left(_)));

        // Left fails (no `i32` extension present), so the right side (`Bot`) is used.
        let right = <Either<Extension<i32>, Bot> as Extractor>::extract(&request)
            .await
            .expect("right side is infallible");
        assert!(matches!(right, Either::Right(_)));

        // Both sides fail -> the last attempt's error is returned.
        let result =
            <Either<Extension<i32>, Extension<String>> as Extractor>::extract(&request).await;
        assert!(result.is_err());
    }
}
