use crate::{
    errors::HandlerError,
    event::service::{service_fn, BoxCloneService, Service},
};

use futures_util::future::BoxFuture;
use std::{
    future::Future,
    task::{Context, Poll},
};

pub type BoxedCloneHandlerService = BoxCloneService<(), (), HandlerError>;

pub type HandlerResult = Result<(), HandlerError>;

pub trait HandlerFn<Args>: Clone + Send + Sync + 'static {
    type Error: Into<anyhow::Error>;
    type Future: Future<Output = Result<(), Self::Error>> + Send;

    fn call(&mut self, args: Args) -> Self::Future;
}

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
