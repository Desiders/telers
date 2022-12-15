use crate::{
    dispatcher::event::service::{
        factory, fn_service, BoxFuture, BoxService, BoxServiceFactory, Service, ServiceFactory,
    },
    error::app,
};

use std::future::Future;

pub type BoxedHandlerService = BoxService<(), (), app::ErrorKind>;
pub type BoxedHandlerServiceFactory = BoxServiceFactory<(), (), (), app::ErrorKind, ()>;

pub trait Handler<Args>: Clone + Send + Sync
where
    Args: Clone + Send + Sync,
{
    type Output;
    type Future: Future<Output = Self::Output> + Send + Sync;

    fn call(&self, args: Args) -> Self::Future;
}

/// [`Handler`] wrapped into service factory with filters
#[allow(clippy::module_name_repetitions)]
pub struct HandlerObject {
    service: BoxedHandlerServiceFactory,
}

impl HandlerObject {
    /// Create a new handler object
    pub fn new<H, Args>(handler: H, args: Args) -> Self
    where
        H: Handler<Args> + 'static,
        Args: Clone + Send + Sync + 'static,
    {
        Self {
            service: handler_service(handler, args),
        }
    }
}

impl ServiceFactory<()> for HandlerObject {
    type Response = ();
    type Error = app::ErrorKind;
    type Config = ();
    type Service = HandlerObjectService;
    type InitError = ();
    type Future = BoxFuture<Result<Self::Service, Self::InitError>>;

    /// Create [`HandlerObjectService`] from [`HandlerObject`]
    fn new_service(&self, _: ()) -> Self::Future {
        let fut = self.service.new_service(());

        Box::pin(async move {
            let service = fut.await?;

            Ok(HandlerObjectService { service })
        })
    }
}

/// [`Handler`] wrapped into service with filters
#[allow(clippy::module_name_repetitions)]
pub struct HandlerObjectService {
    service: BoxedHandlerService,
}

impl Service<()> for HandlerObjectService {
    type Response = ();
    type Error = app::ErrorKind;
    type Future = BoxFuture<Result<Self::Response, Self::Error>>;

    /// Call service, which is wrapped [`Handler`]
    fn call(&self, req: ()) -> Self::Future {
        self.service.call(req)
    }
}

/// Wrap [`Handler`] into service factory
#[allow(clippy::module_name_repetitions)]
pub fn handler_service<H, Args>(handler: H, args: Args) -> BoxedHandlerServiceFactory
where
    H: Handler<Args> + 'static,
    Args: Clone + Send + Sync + 'static,
{
    factory(fn_service(move |()| {
        let handler = handler.clone();
        let args = args.clone();

        async move {
            handler.call(args).await;
            Ok(())
        }
    }))
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
    impl<Func, Fut, $($param,)*> Handler<($($param,)*)> for Func
    where
        Func: Fn($($param),*) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future + Send + Sync + 'static,
        $($param: Clone + Send + Sync + 'static,)*
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

factory_tuple! {}
factory_tuple! { A }
factory_tuple! { A B }
factory_tuple! { A B C }
factory_tuple! { A B C D }
factory_tuple! { A B C D E }
factory_tuple! { A B C D E F }
factory_tuple! { A B C D E F G }
factory_tuple! { A B C D E F G H }
factory_tuple! { A B C D E F G H I }
factory_tuple! { A B C D E F G H I J }
factory_tuple! { A B C D E F G H I J K }
factory_tuple! { A B C D E F G H I J K L }

#[cfg(test)]
mod tests {
    use super::*;

    use tokio;

    #[test]
    fn test_arg_number() {
        fn assert_impl_handler<T: Clone + Send + Sync>(_: impl Handler<T>) {}

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
        let handler_object = HandlerObject::new(|| async {}, ());
        let handler_object_service = handler_object.new_service(()).await.unwrap();

        handler_object_service.call(()).await.unwrap();
    }
}
