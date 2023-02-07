use crate::{
    dispatcher::event::service::{
        factory, fn_service, BoxFuture, BoxService, BoxServiceFactory, Service, ServiceFactory,
    },
    error::EventError,
};

use std::{future::Future, result::Result as StdResult};

pub type BoxedHandlerService = BoxService<(), (), EventError>;
pub type BoxedHandlerServiceFactory = BoxServiceFactory<(), (), (), EventError, ()>;

pub type Result = StdResult<(), EventError>;

pub trait Handler<Args> {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn call(&self, args: Args) -> Self::Future;
}

#[allow(clippy::module_name_repetitions)]
pub struct HandlerObject {
    service: BoxedHandlerServiceFactory,
}

impl HandlerObject {
    #[must_use]
    pub fn new<H, Args>(handler: H, args: Args) -> Self
    where
        H: Handler<Args> + Clone + Send + Sync + 'static,
        H::Future: Send,
        H::Output: Into<Result>,
        Args: Clone + Send + Sync + 'static,
    {
        Self {
            service: handler_service(handler, args),
        }
    }
}

impl ServiceFactory<()> for HandlerObject {
    type Response = ();
    type Error = EventError;
    type Config = ();
    type Service = HandlerObjectService;
    type InitError = ();

    fn new_service(&self, config: Self::Config) -> StdResult<Self::Service, Self::InitError> {
        let service = self.service.new_service(config)?;

        Ok(HandlerObjectService { service })
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct HandlerObjectService {
    service: BoxedHandlerService,
}

impl Service<()> for HandlerObjectService {
    type Response = ();
    type Error = EventError;
    type Future = BoxFuture<StdResult<Self::Response, Self::Error>>;

    fn call(&self, req: ()) -> Self::Future {
        self.service.call(req)
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn handler_service<H, Args>(handler: H, args: Args) -> BoxedHandlerServiceFactory
where
    H: Handler<Args> + Clone + Send + Sync + 'static,
    H::Future: Send,
    H::Output: Into<Result>,
    Args: Clone + Send + Sync + 'static,
{
    factory(fn_service(move |()| {
        let handler = handler.clone();
        let args = args.clone();

        async move { handler.call(args).await.into() }
    }))
}

#[allow(non_snake_case)]
#[doc(hidden)]
mod factory_handlers {
    use super::{Future, Handler};

    // `Handler` implementation for functions
    macro_rules! factory ({ $($param:ident)* } => {
        impl<Func, Fut, $($param,)*> Handler<($($param,)*)> for Func
        where
            Func: Fn($($param),*) -> Fut,
            Fut: Future,
        {
            type Output = Fut::Output;
            type Future = Fut;

            #[inline]
            #[allow(non_snake_case)]
            fn call(&self, ($($param,)*): ($($param,)*)) -> Self::Future {
                (self)($($param,)*)
            }
        }
    });

    // To be able to use function without arguments
    factory! {}
    // To be able to use function with 1 arguments
    factory! { A }
    // To be able to use function with 2 arguments
    factory! { A B }
    // To be able to use function with 3 arguments
    factory! { A B C }
    // To be able to use function with 4 arguments
    factory! { A B C D }
    // To be able to use function with 5 arguments
    factory! { A B C D E }
    // To be able to use function with 6 arguments
    factory! { A B C D E F }
    // To be able to use function with 7 arguments
    factory! { A B C D E F G }
    // To be able to use function with 8 arguments
    factory! { A B C D E F G H }
    // To be able to use function with 9 arguments
    factory! { A B C D E F G H I }
    // To be able to use function with 10 arguments
    factory! { A B C D E F G H I J }
    // To be able to use function with 11 arguments
    factory! { A B C D E F G H I J K }
    // To be able to use function with 12 arguments
    factory! { A B C D E F G H I J K L }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tokio;

    #[test]
    fn test_arg_number() {
        fn assert_impl_handler<T>(_: impl Handler<T>) {}

        assert_impl_handler(|| async { unreachable!() });
        assert_impl_handler(
            |_01: (),
             _02: (),
             _03: (),
             _04: (),
             _05: (),
             _06: (),
             _07: (),
             _08: (),
             _09: (),
             _10: (),
             _11: (),
             _12: ()| async { unreachable!() },
        );
    }

    #[tokio::test]
    async fn test_handler_object_service() {
        let handler_object = HandlerObject::new(|| async { Ok(()) }, ());
        let handler_object_service = handler_object.new_service(()).unwrap();

        let res = handler_object_service.call(()).await;

        match res {
            Ok(_) => {}
            _ => panic!("Unexpected result"),
        }
    }
}
