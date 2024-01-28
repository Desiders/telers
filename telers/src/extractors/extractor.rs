use crate::{
    client::{Bot, Reqwest},
    context::Context,
    errors::ExtractionError,
    types::Update,
};

use std::{convert::Infallible, sync::Arc};

/// Trait for extracting data from [`Update`] and [`Context`] to handlers arguments
pub trait FromEventAndContext<Client = Reqwest>: Sized {
    type Error: Into<ExtractionError>;

    /// Extracts data from [`Update`], [`Context`] and [`Bot`] to handler argument
    /// # Returns
    /// [`Ok(Self)`] if extraction was successful and [`Err(Self::Error)`] otherwise
    /// # Errors
    /// If extraction was unsuccessful
    /// Possible variants:
    /// * No found data in context by key
    /// * Data in context by key has wrong type. For example, you try to extract `i32` from `String`.
    /// * Custom user error
    fn extract(
        bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error>;
}

/// To be able to use [`Option`] as handler argument
/// This implementation will return `None` if extraction was unsuccessful, and [`Some(value)`] otherwise
/// Useful for optional arguments
impl<Client, T: FromEventAndContext<Client>> FromEventAndContext<Client> for Option<T> {
    type Error = Infallible;

    fn extract(
        bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        match T::extract(bot, update, context) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None),
        }
    }
}

/// To be able to use [`Result`] as handler argument
/// This implementation will return [`Ok(value)`] if extraction was successful, and [`Err(error)`] otherwise,
/// where `error` is `T::Error` converted to `E`
/// Useful for optional arguments
impl<Client, T, E> FromEventAndContext<Client> for Result<T, E>
where
    T: FromEventAndContext<Client>,
    T::Error: Into<E>,
{
    type Error = Infallible;

    fn extract(
        bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        Ok(T::extract(bot, update, context).map_err(Into::into))
    }
}

/// To be able to use handler without arguments
/// Handler without arguments will be called with [`()`] argument (unit type)
impl<Client> FromEventAndContext<Client> for () {
    type Error = Infallible;

    fn extract(
        _bot: Arc<Bot<Client>>,
        _update: Arc<Update>,
        _context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        Ok(())
    }
}

#[allow(non_snake_case)]
mod factory_from_event_and_context {
    //! This module is used to implement [`FromEventAndContext`] for tuple arguments, each of which implements it
    //! If one of the arguments fails to extract, the whole extraction fails, and the error is returned

    use super::{Arc, Bot, Context, ExtractionError, FromEventAndContext, Update};

    macro_rules! factory ({ $($param:ident)* } => {
        impl<Client, $($param: FromEventAndContext<Client>,)*> FromEventAndContext<Client> for ($($param,)*) {
            type Error = ExtractionError;

            #[inline]
            fn extract(bot: Arc<Bot<Client>>, update: Arc<Update>, context: Arc<Context>) -> Result<Self, Self::Error> {
                Ok(($($param::extract(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context)).map_err(Into::into)?,)*))
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
    // To be able to extract tuple with 17 arguments
    factory! { A B C D E F G H I J K L M N O P Q}
    // To be able to extract tuple with 18 arguments
    factory! { A B C D E F G H I J K L M N O P Q R }
    // To be able to extract tuple with 19 arguments
    factory! { A B C D E F G H I J K L M N O P Q R S }
    // To be able to extract tuple with 20 arguments
    factory! { A B C D E F G H I J K L M N O P Q R S T }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::{Bot, Reqwest},
        context::Context,
        types::Update,
    };

    #[test]
    fn test_arg_number() {
        fn assert_impl_handler<Client, T: FromEventAndContext<Client>>(_: T) {}

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
            (), // 17
            (), // 18
            (), // 19
            (), // 20
        ));
    }

    #[test]
    fn test_extract() {
        let bot = Arc::new(Bot::<Reqwest>::default());
        let update = Arc::new(Update::default());
        let context = Arc::new(Context::default());

        let (): () =
            FromEventAndContext::extract(bot.clone(), update.clone(), context.clone()).unwrap();
        let _: Option<()> =
            FromEventAndContext::extract(bot.clone(), update.clone(), context.clone()).unwrap();
        let _: Result<(), Infallible> =
            FromEventAndContext::extract(bot.clone(), update.clone(), context.clone()).unwrap();
    }
}
