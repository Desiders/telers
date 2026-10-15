//! Result of a Telegram handler run and the conversions that let a handler return it.
//!
//! A handler function may return an [`EventReturn`], a `()` or an [`Infallible`] directly, and
//! any of those is converted by [`IntoHandlerResult`] before it reaches the observer.

use crate::{client::Reqwest, errors::HandlerError, event::EventReturn, Request};

use std::{
    convert::Infallible,
    fmt::{self, Debug, Formatter},
};

/// What a Telegram handler returns: the [`EventReturn`] that tells the observer how to continue,
/// or an error from the handler.
///
/// It defaults to `Result<EventReturn, HandlerError>` and is the type of the
/// [`Response::result`] field.
#[allow(type_alias_bounds)]
pub type HandlerResult<T: Into<EventReturn> = EventReturn, E: Into<anyhow::Error> = HandlerError> =
    Result<T, E>;

/// What a Telegram handler gives back to the observer: the request it was called with and the
/// result of the run.
pub struct Response<Client = Reqwest> {
    pub request: Request<Client>,
    /// What the handler returned: the [`EventReturn`] that tells the observer how to continue,
    /// or the error it failed with.
    pub result: HandlerResult,
}

impl<Client> Debug for Response<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("request", &self.request)
            .field("result", &self.result)
            .finish()
    }
}

/// Conversion of a handler return type into a [`HandlerResult`].
///
/// It is implemented for [`EventReturn`] and `()`, which become a finished propagation, for
/// [`Infallible`], which never fails, and for a `Result` whose `Ok` converts into [`EventReturn`]
/// and whose error is wrapped into a [`HandlerError`].
pub trait IntoHandlerResult {
    #[allow(clippy::missing_errors_doc)]
    fn into_handler_result(self) -> HandlerResult;
}

impl IntoHandlerResult for EventReturn {
    #[inline]
    fn into_handler_result(self) -> HandlerResult {
        Ok(self)
    }
}

impl IntoHandlerResult for () {
    #[inline]
    fn into_handler_result(self) -> HandlerResult {
        Ok(EventReturn::Finish)
    }
}

impl IntoHandlerResult for Infallible {
    #[inline(never)]
    fn into_handler_result(self) -> HandlerResult {
        match self {}
    }
}

impl<T, E> IntoHandlerResult for Result<T, E>
where
    T: Into<EventReturn>,
    E: Into<anyhow::Error>,
{
    fn into_handler_result(self) -> HandlerResult {
        match self {
            Ok(val) => Ok(val.into()),
            Err(err) => Err(HandlerError::new(err)),
        }
    }
}
