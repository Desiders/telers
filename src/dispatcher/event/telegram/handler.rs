use crate::{
    client::Bot,
    context::Context,
    dispatcher::event::{
        bases::EventReturn,
        service::{
            factory, fn_service, BoxFuture, BoxService, BoxServiceFactory, Service, ServiceFactory,
        },
    },
    error::app,
    extract::FromEventAndContext,
    filters::base::Filter,
    types::Update,
};

use std::{future::Future, sync::Arc};

pub type BoxedHandlerService = BoxService<Request, Response, app::ErrorKind>;
pub type BoxedHandlerServiceFactory = BoxServiceFactory<(), Request, Response, app::ErrorKind, ()>;

/// Data for handler service
#[derive(Clone, Debug)]
pub struct Request {
    bot: Arc<Bot>,
    /// Update from Telegram
    update: Arc<Update>,
    /// Context, which can contain some data. Can be mapped to handler arguments,
    /// used as hashmap in handlers, middlewares and filters
    context: Arc<Context>,
}

impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.bot, &other.bot)
            && Arc::ptr_eq(&self.update, &other.update)
            && Arc::ptr_eq(&self.context, &other.context)
    }
}

impl Request {
    #[must_use]
    pub fn new<B: Into<Arc<Bot>>, U: Into<Arc<Update>>, C: Into<Arc<Context>>>(
        bot: B,
        update: U,
        context: C,
    ) -> Self {
        Self {
            bot: bot.into(),
            update: update.into(),
            context: context.into(),
        }
    }

    #[must_use]
    pub fn bot(&self) -> Arc<Bot> {
        Arc::clone(&self.bot)
    }

    #[must_use]
    pub fn update(&self) -> Arc<Update> {
        Arc::clone(&self.update)
    }

    #[must_use]
    pub fn context(&self) -> Arc<Context> {
        Arc::clone(&self.context)
    }
}

/// Response from handler service.
/// For the response from handler for users, use [`EventReturn`].
#[derive(Clone, PartialEq, Debug)]
pub struct Response {
    request: Request,
    response: EventReturn,
}

impl Response {
    #[must_use]
    pub fn new(request: Request, response: EventReturn) -> Self {
        Self { request, response }
    }

    #[must_use]
    pub fn request(&self) -> &Request {
        &self.request
    }

    #[must_use]
    pub fn response(&self) -> &EventReturn {
        &self.response
    }
}

pub trait Handler<Args>: Clone + Send + Sync
where
    Args: Send + Sync,
{
    type Output;
    type Future: Future<Output = Self::Output> + Send + Sync;

    fn call(&self, args: Args) -> Self::Future;
}

/// [`Handler`] wrapped into service factory with filters
#[allow(clippy::module_name_repetitions)]
pub struct HandlerObject {
    service: BoxedHandlerServiceFactory,
    pub(crate) filters: Arc<Vec<Box<dyn Filter>>>,
}

impl HandlerObject {
    /// Creates a new handler object
    pub fn new<H, Args>(handler: H, filters: Vec<Box<dyn Filter>>) -> Self
    where
        H: Handler<Args> + 'static,
        Args: FromEventAndContext + 'static,
        H::Output: Into<EventReturn>,
    {
        Self {
            service: handler_service(handler),
            filters: Arc::new(filters),
        }
    }

    /// Get filters of the handler
    #[must_use]
    pub fn filters(&self) -> &[Box<dyn Filter>] {
        &self.filters
    }

    /// Register filter for the handler
    /// # Arguments
    /// * `filter` - Filter for the handler
    /// # Panics
    /// If there are other [`Arc`] or `Weak` pointers to the same allocation
    pub fn filter(&mut self, filter: Box<dyn Filter>) {
        Arc::get_mut(&mut self.filters).unwrap().push(filter);
    }
}

impl ServiceFactory<Request> for HandlerObject {
    type Response = Response;
    type Error = app::ErrorKind;
    type Config = ();
    type Service = HandlerObjectService;
    type InitError = ();
    type Future = BoxFuture<Result<Self::Service, Self::InitError>>;

    /// Create [`HandlerObjectService`] from [`HandlerObject`]
    fn new_service(&self, _: ()) -> Self::Future {
        let fut = self.service.new_service(());
        let filters = Arc::clone(&self.filters);

        Box::pin(async move {
            let service = fut.await?;

            Ok(HandlerObjectService {
                service: Arc::new(service),
                filters,
            })
        })
    }
}

/// [`Handler`] wrapped into service with filters
#[allow(clippy::module_name_repetitions)]
pub struct HandlerObjectService {
    service: Arc<BoxedHandlerService>,
    filters: Arc<Vec<Box<dyn Filter>>>,
}

impl HandlerObjectService {
    /// Check if the handler pass the filters.
    /// If the handler pass all them, it will be called.
    #[must_use]
    pub fn check(&self, req: &Request) -> bool {
        self.filters
            .iter()
            .all(|filter| filter.check(&req.bot, &req.update, &req.context))
    }

    #[must_use]
    pub fn service(&self) -> Arc<BoxedHandlerService> {
        Arc::clone(&self.service)
    }
}

impl Service<Request> for HandlerObjectService {
    type Response = Response;
    type Error = app::ErrorKind;
    type Future = BoxFuture<Result<Self::Response, Self::Error>>;

    /// Call service, which is wrapped [`Handler`]
    fn call(&self, req: Request) -> Self::Future {
        self.service.call(req)
    }
}

/// Wrap [`Handler`] into service factory
#[allow(clippy::module_name_repetitions)]
pub fn handler_service<H, Args>(handler: H) -> BoxedHandlerServiceFactory
where
    H: Handler<Args> + 'static,
    Args: FromEventAndContext + 'static,
    H::Output: Into<EventReturn>,
{
    factory(fn_service(move |req: Request| {
        let handler = handler.clone();

        async move {
            match Args::extract(req.bot(), req.update(), req.context()).await {
                // Call the handler with extracted arguments
                Ok(args) => Ok(Response {
                    request: req,
                    response: handler.call(args).await.into(),
                }),
                // Return error which implement `Into<app::ErrorKind>`
                Err(err) => Err(err.into()),
            }
        }
    }))
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
    impl<Func, Fut, $($param,)*> Handler<($($param,)*)> for Func
    where
        Func: Fn($($param),*) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future + Send + Sync + 'static,
        $($param: Send + Sync + 'static,)*
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
    use crate::filters::command;

    use tokio;

    #[test]
    fn test_arg_number() {
        fn assert_impl_handler<T: FromEventAndContext>(_: impl Handler<T>) {}

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

    #[test]
    fn test_handler_object_filter() {
        let filter = Box::new(command::Command {
            commands: vec![command::PatternType::Text("start")],
            prefix: "/",
            ignore_case: false,
            ignore_mention: false,
        });

        let mut handler_object = HandlerObject::new(|| async { unreachable!() }, vec![]);
        assert_eq!(handler_object.filters().is_empty(), true);

        handler_object.filter(filter.clone());
        assert_eq!(handler_object.filters().len(), 1);

        let handler_object = HandlerObject::new(|| async { unreachable!() }, vec![filter.clone()]);
        assert_eq!(handler_object.filters().len(), 1);
    }

    #[tokio::test]
    async fn test_handler_object_service() {
        let handler_object = HandlerObject::new(|| async {}, vec![]);
        let handler_object_service = handler_object.new_service(()).await.unwrap();

        let req = Request::new(Bot::default(), Update::default(), Context::new());
        assert_eq!(handler_object_service.check(&req), true);

        let res = handler_object_service.call(req).await.unwrap();

        assert_eq!(res.response().is_cancel(), false);
        assert_eq!(res.response().is_skip(), false);
    }
}
