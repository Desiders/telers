use futures_util::future::BoxFuture;

use crate::{
    errors::HandlerError,
    event::service::{service_fn, BoxCloneService, Service},
};

use std::{
    future::Future,
    task::{Context, Poll},
};

pub type BoxedCloneHandlerService = BoxCloneService<(), (), HandlerError>;

pub type HandlerResult = Result<(), HandlerError>;

pub trait Handler<Args>: Clone + Send + Sync + 'static {
    type Output: Into<HandlerResult>;
    type Future: Future<Output = Self::Output> + Send;

    fn call(&mut self, args: Args) -> Self::Future;
}

#[derive(Clone)]
pub struct HandlerComposite {
    service: BoxedCloneHandlerService,
}

impl HandlerComposite {
    #[must_use]
    pub fn new<H, Args>(handler: H, args: Args) -> Self
    where
        H: Handler<Args>,
        Args: Clone + Send + Sync + 'static,
    {
        Self {
            service: boxed_handler_factory(handler, args),
        }
    }
}

impl Service<()> for HandlerComposite {
    type Response = ();
    type Error = HandlerError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: ()) -> Self::Future {
        self.service.call(req)
    }
}

pub fn boxed_handler_factory<H, Args>(handler: H, args: Args) -> BoxedCloneHandlerService
where
    H: Handler<Args>,
    Args: Clone + Send + Sync + 'static,
{
    BoxCloneService::new(service_fn(move |()| {
        let mut handler = handler.clone();
        let args = args.clone();

        async move { handler.call(args).await.into() }
    }))
}

macro_rules! impl_handlers {
    (
        [$($ty:ident),*]
    ) => {
        impl<F, Fut, Response, $($ty,)*> Handler<($($ty,)*)> for F
        where
            F: FnMut($($ty),*) -> Fut + Clone + Send + Sync + 'static,
            Fut: Future<Output = Response> + Send,
            Response: Into<HandlerResult>,
        {
            type Output = Response;
            type Future = Fut;

            #[inline]
            #[allow(non_snake_case)]
            fn call(&mut self, ($($ty,)*): ($($ty,)*)) -> Self::Future {
                (self)($($ty,)*)
            }
        }
    }
}

all_the_tuples!(impl_handlers);
