use super::event::ErrorEvent;
use crate::{
    errors::HandlerError,
    event::{service::Service, EventReturn},
};

use futures_util::future::BoxFuture;
use std::{
    future::Future,
    task::{Context, Poll},
};

pub type HandlerResult = Result<EventReturn, HandlerError>;

/// Error handler function that takes the [`ErrorEvent`] and returns [`HandlerResult`].
///
/// Implemented for any `FnMut(ErrorEvent) -> Future<Output = HandlerResult>` closure or function.
pub trait HandlerFn<Client>: Clone + Send + Sync + 'static {
    type Future: Future<Output = HandlerResult> + Send;

    /// Call the handler with the given error event.
    fn call(&mut self, event: ErrorEvent<Client>) -> Self::Future;
}

pub struct Handler<Client> {
    service: BoxedCloneHandlerService<Client>,
}

impl<Client> Clone for Handler<Client> {
    fn clone(&self) -> Self {
        Self {
            service: self.service.clone(),
        }
    }
}

pub(crate) type BoxedCloneHandlerService<Client> =
    crate::event::service::BoxCloneService<ErrorEvent<Client>, EventReturn, HandlerError>;

impl<Client> Handler<Client>
where
    Client: Send + Sync + 'static,
{
    /// Create a new error handler from a function or closure
    /// that implements [`HandlerFn`].
    #[must_use]
    pub fn new<F>(handler_fn: F) -> Self
    where
        F: HandlerFn<Client>,
    {
        Self {
            service: boxed_handler_factory(handler_fn),
        }
    }
}

impl<Client> Service<ErrorEvent<Client>> for Handler<Client>
where
    Client: Send + Sync + 'static,
{
    type Error = HandlerError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = EventReturn;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn call(&mut self, req: ErrorEvent<Client>) -> Self::Future {
        self.service.call(req)
    }
}

pub(crate) fn boxed_handler_factory<Client, F>(handler_fn: F) -> BoxedCloneHandlerService<Client>
where
    Client: Send + Sync + 'static,
    F: HandlerFn<Client>,
{
    crate::event::service::BoxCloneService::new(crate::event::service::service_fn(
        move |event: ErrorEvent<Client>| {
            let mut handler_fn = handler_fn.clone();

            Box::pin(async move { handler_fn.call(event).await })
        },
    ))
}

impl<F, Fut, Client> HandlerFn<Client> for F
where
    F: FnMut(ErrorEvent<Client>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = HandlerResult> + Send,
    Client: Send + Sync + 'static,
{
    type Future = Fut;

    #[inline]
    fn call(&mut self, event: ErrorEvent<Client>) -> Self::Future {
        (self)(event)
    }
}
