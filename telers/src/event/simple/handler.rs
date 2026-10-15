//! Handlers of the [simple observer](super::Observer), used for events like startup and shutdown.
//!
//! A [`Handler`] keeps the arguments its function is called with, so the observer can trigger it
//! without extracting anything from a request, and every handler returns [`HandlerResult`].

use crate::{
    errors::HandlerError,
    event::service::{service_fn, BoxCloneService, Service},
};

use futures_util::future::BoxFuture;
use std::{
    future::Future,
    task::{Context, Poll},
};

/// Type-erased handler: a clonable service that takes no arguments and returns `()`.
pub type BoxedCloneHandlerService = BoxCloneService<(), (), HandlerError>;

/// What a simple observer handler returns, where any error stops the observer.
pub type HandlerResult = Result<(), HandlerError>;

/// Adapter that lets a function or closure be used as a simple observer handler.
///
/// It is implemented for every `FnMut` taking up to a fixed number of arguments and returning a
/// future of `Result<(), E>`, and the arguments of one call are stored in the [`Handler`].
pub trait HandlerFn<Args>: Clone + Send + Sync + 'static {
    type Error: Into<anyhow::Error>;
    type Future: Future<Output = Result<(), Self::Error>> + Send;

    fn call(&mut self, args: Args) -> Self::Future;
}

/// A simple observer handler: the function to call and the arguments to call it with.
///
/// Build it with [`Handler::new`] from a function or closure, or with [`Handler::new_service`]
/// from a [`Service`]; the observer triggers it as a [`Service`] with an empty request.
#[derive(Clone)]
pub struct Handler {
    service: BoxedCloneHandlerService,
}

impl Handler {
    #[must_use]
    pub fn new<H, Args>(handler: H, args: Args) -> Self
    where
        H: HandlerFn<Args>,
        Args: Clone + Send + Sync + 'static,
    {
        Self {
            service: boxed_handler_factory(handler, args),
        }
    }

    #[must_use]
    pub fn new_service<S, Args>(service: S, args: Args) -> Self
    where
        S: Service<Args, Response = ()> + Clone + Send + Sync + 'static,
        S::Error: Into<anyhow::Error>,
        S::Future: Send,
        Args: Clone + Send + Sync + 'static,
    {
        Self {
            service: boxed_service_factory(service, args),
        }
    }
}

impl Service<()> for Handler {
    type Error = HandlerError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = ();

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn call(&mut self, req: ()) -> Self::Future {
        self.service.call(req)
    }
}

/// Box a [`HandlerFn`] and its arguments into a [`BoxedCloneHandlerService`].
///
/// # Arguments
/// * `handler` - Function or closure to call on every trigger of the service.
/// * `args` - Arguments passed to the function on every call.
pub fn boxed_handler_factory<H, Args>(handler: H, args: Args) -> BoxedCloneHandlerService
where
    H: HandlerFn<Args>,
    Args: Clone + Send + Sync + 'static,
{
    BoxCloneService::new(service_fn(move |()| {
        let mut handler = handler.clone();
        let args = args.clone();

        async move { handler.call(args).await.map_err(HandlerError::new) }
    }))
}

/// Box a [`Service`] and its arguments into a [`BoxedCloneHandlerService`].
///
/// # Arguments
/// * `service` - Service to call on every trigger of the boxed service.
/// * `args` - Arguments passed to the service on every call.
pub fn boxed_service_factory<S, Args>(service: S, args: Args) -> BoxedCloneHandlerService
where
    S: Service<Args, Response = ()> + Clone + Send + Sync + 'static,
    S::Error: Into<anyhow::Error>,
    S::Future: Send,
    Args: Clone + Send + Sync + 'static,
{
    BoxCloneService::new(service_fn(move |()| {
        let mut service = service.clone();
        let args = args.clone();

        async move { service.call(args).await.map_err(HandlerError::new) }
    }))
}

macro_rules! impl_handlers {
    (
        [$($ty:ident),*]
    ) => {
        impl<F, Fut, Err, $($ty,)*> HandlerFn<($($ty,)*)> for F
        where
            F: FnMut($($ty),*) -> Fut + Clone + Send + Sync + 'static,
            Err: Into<anyhow::Error>,
            Fut: Future<Output = Result<(), Err>> + Send,
        {
            type Error = Err;
            type Future = Fut;

            #[allow(non_snake_case)]
            fn call(&mut self, ($($ty,)*): ($($ty,)*)) -> Self::Future {
                (self)($($ty,)*)
            }
        }
    }
}

all_the_tuples!(impl_handlers);

#[cfg(test)]
mod tests {
    use super::*;

    use std::convert::Infallible;
    use tokio;

    #[tokio::test]
    async fn test_handler() {
        let mut handler = Handler::new(|(), ()| async { Ok::<_, Infallible>(()) }, ((), ()));

        handler.call(()).await.unwrap();
    }

    #[tokio::test]
    async fn test_service() {
        let mut handler = Handler::new_service(
            service_fn(|((), ())| async { Ok::<_, Infallible>(()) }),
            ((), ()),
        );

        handler.call(()).await.unwrap();
    }
}
