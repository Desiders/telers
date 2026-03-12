use crate::{client::Reqwest, errors::HandlerError, event::EventReturn, Request};

use std::{
    convert::Infallible,
    fmt::{self, Debug, Formatter},
};

#[allow(type_alias_bounds)]
pub type HandlerResult<T: Into<EventReturn> = EventReturn, E: Into<anyhow::Error> = HandlerError> =
    Result<T, E>;

pub struct Response<Client = Reqwest> {
    pub request: Request<Client>,
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
