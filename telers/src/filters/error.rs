//! Filters for the `error` observer, see the [`router module`] for how errors are handled.
//!
//! - [`ErrorType`] checks the type of the error
//! - [`ErrorMessage`] checks the text of the error
//!
//! [`router module`]: telers::router

use super::{text::PatternType, Filter, FilterResult};
use crate::{errors::EventErrorKind, Request};

use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    future::Future,
    marker::PhantomData,
};

/// Filter for checking the type of the error in the `error` observer.
/// It passes if the error can be downcast to `E`, see [`EventErrorKind::downcast_ref`].
/// Usually used with the [`EventError`](crate::errors::EventError) handler argument.
/// # Notes
/// Outside the `error` observer there is no error in the request, so the filter doesn't pass.
///
/// To check several types, combine the filters with [`Filter::or`].
pub struct ErrorType<E> {
    marker: PhantomData<E>,
}

impl<E> ErrorType<E> {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<E> Default for ErrorType<E> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<E> Clone for ErrorType<E> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<Client, E> Filter<Client> for ErrorType<E>
where
    E: Display + Debug + Send + Sync + 'static,
{
    type Error = Infallible;

    fn check(
        &mut self,
        request: &mut Request<Client>,
    ) -> impl Future<Output = FilterResult<Self::Error>> + Send {
        let res = request
            .extensions
            .get::<EventErrorKind>()
            .is_some_and(|err| err.downcast_ref::<E>().is_some());
        async move { Ok(res) }
    }
}

/// Filter for checking the text of the error in the `error` observer.
/// The text is the [`Display`] representation of the error, and it must be equal to one of the texts
/// or match one of the [`Regex`](regex::Regex) patterns.
/// # Notes
/// Outside the `error` observer there is no error in the request, so the filter doesn't pass.
#[derive(Debug, Clone)]
pub struct ErrorMessage {
    /// List of texts or compiled [`Regex`](regex::Regex) patterns that must be equal to the text of the error
    texts: Vec<PatternType>,
}

impl ErrorMessage {
    /// # Arguments
    /// * `text` - Text that must be equal to the text of the error or compiled [`Regex`](regex::Regex) pattern that must match it
    #[must_use]
    pub fn one(text: impl Into<PatternType>) -> Self {
        Self::many([text])
    }

    /// # Arguments
    /// * `texts` - Texts that must be equal to the text of the error or compiled [`Regex`](regex::Regex) patterns that must match it
    #[must_use]
    pub fn many(texts: impl IntoIterator<Item = impl Into<PatternType>>) -> Self {
        Self {
            texts: texts.into_iter().map(Into::into).collect(),
        }
    }

    fn validate(&self, text: &str) -> bool {
        self.texts.iter().any(|pattern| match pattern {
            PatternType::Text(allowed_text) => allowed_text.as_ref() == text,
            PatternType::Regex(regex) => regex.is_match(text),
        })
    }
}

impl<Client> Filter<Client> for ErrorMessage {
    type Error = Infallible;

    fn check(
        &mut self,
        request: &mut Request<Client>,
    ) -> impl Future<Output = FilterResult<Self::Error>> + Send {
        let res = request
            .extensions
            .get::<EventErrorKind>()
            .is_some_and(|err| self.validate(&err.to_string()));
        async move { Ok(res) }
    }
}
