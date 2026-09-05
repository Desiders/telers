//! This module contains the [`CallbackDataFilter`] filter, which is used to filter
//! [`CallbackQuery`] updates with callback data that can be unpacked to the given
//! [`CallbackData`] type.

use super::{Filter, FilterResult};
use crate::{callback_data::CallbackData, Request};

use std::{convert::Infallible, future::Future, marker::PhantomData};

/// Filter for [`CallbackQuery`] updates with callback data that can be unpacked
/// to the given [`CallbackData`] type.
///
/// On success, the unpacked data is placed in the request context by the [`CallbackData`] key,
/// so it can be extracted in handlers. The [`CallbackData`] derive macro generates
/// the extractor implementation for this.
///
/// If the callback data can't be unpacked to the given type
/// (for example, it belongs to another type or was corrupted), the filter returns `false`.
///
/// # Examples
///
/// ```rust
/// use telers::{filters::callback_data::CallbackDataFilter, CallbackData, Router};
///
/// #[derive(CallbackData, Clone)]
/// #[callback_data(prefix = "language")]
/// struct LanguageSettings {
///     language_code: String,
///     enabled: bool,
/// }
///
/// // Only callback queries with data unpackable to `LanguageSettings` will pass the filter
/// let router: Router = Router::new("language settings").on_callback_query(|observer| {
///     observer.filter(CallbackDataFilter::<LanguageSettings>::new())
/// });
/// ```
///
/// [`CallbackQuery`]: crate::types::CallbackQuery
#[derive(Debug, Clone, Copy, Default)]
pub struct CallbackDataFilter<CD> {
    phantom: PhantomData<CD>,
}

impl<CD> CallbackDataFilter<CD>
where
    CD: CallbackData,
{
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<Client, CD> Filter<Client> for CallbackDataFilter<CD>
where
    Client: Send,
    CD: CallbackData + Clone + Send + Sync + 'static,
{
    type Error = Infallible;

    fn check(
        &mut self,
        request: &mut Request<Client>,
    ) -> impl Future<Output = FilterResult<Self::Error>> {
        let res = match request.update.callback_query() {
            Some(callback_query) => match callback_query.data.as_deref().map(CD::unpack) {
                Some(Ok(callback_data)) => {
                    request.context.insert("callback_data", callback_data);
                    true
                }
                _ => false,
            },
            None => false,
        };
        async move { Ok(res) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        types::{CallbackQuery, Update, UpdateCallbackQuery, User},
        Bot, CallbackData, Request,
    };

    use std::sync::Arc;

    #[derive(Debug, Clone, PartialEq, CallbackData)]
    #[callback_data(prefix = "test")]
    struct TestData {
        value: i32,
    }

    fn request(data: Option<&str>) -> Request<crate::client::Reqwest> {
        Request {
            bot: Bot::default(),
            update: Arc::new(Update::CallbackQuery(UpdateCallbackQuery::new(
                0,
                CallbackQuery::new("id", User::new(1, true, "test"), "chat instance")
                    .data(data.unwrap_or_default()),
            ))),
            context: crate::Context::default(),
            extensions: crate::Extensions::default(),
        }
    }

    #[tokio::test]
    async fn test_filter_passes_and_inserts_to_context() {
        let mut filter = CallbackDataFilter::<TestData>::new();
        let mut request = request(Some("test:42"));

        assert!(filter.check(&mut request).await.unwrap());
        assert_eq!(
            request.context.get::<TestData>("callback_data"),
            Some(&TestData {
                value: 42
            })
        );
    }

    #[tokio::test]
    async fn test_filter_fails_on_invalid_data() {
        let mut filter = CallbackDataFilter::<TestData>::new();

        let mut other_prefix_request = request(Some("other_prefix:42"));
        assert!(!filter.check(&mut other_prefix_request).await.unwrap());

        let mut invalid_number_request = request(Some("test:not_a_number"));
        assert!(!filter.check(&mut invalid_number_request).await.unwrap());

        let mut missing_value_request = request(Some("test"));
        assert!(!filter.check(&mut missing_value_request).await.unwrap());
    }

    #[tokio::test]
    async fn test_filter_fails_on_other_update() {
        let mut filter = CallbackDataFilter::<TestData>::new();
        let mut request: Request<crate::client::Reqwest> = Request {
            bot: Bot::default(),
            update: Arc::new(Update::Message(crate::types::UpdateMessage::new(
                0,
                crate::types::MessageText::new(0, 0, crate::types::ChatPrivate::new(0), "test"),
            ))),
            context: crate::Context::default(),
            extensions: crate::Extensions::default(),
        };

        assert!(!filter.check(&mut request).await.unwrap());
    }
}
