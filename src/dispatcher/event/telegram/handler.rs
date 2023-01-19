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

/// Request for a handler service
#[derive(Clone, Debug)]
pub struct Request {
    bot: Arc<Bot>,
    update: Arc<Update>,
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
    pub fn new<B, U, C>(bot: B, update: U, context: C) -> Self
    where
        B: Into<Arc<Bot>>,
        U: Into<Arc<Update>>,
        C: Into<Arc<Context>>,
    {
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

/// Response from handler service
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

pub trait Handler<Args> {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn call(&self, args: Args) -> Self::Future;
}

#[allow(clippy::module_name_repetitions)]
pub struct HandlerObject {
    service: BoxedHandlerServiceFactory,
    pub(crate) filters: Arc<Vec<Box<dyn Filter>>>,
}

impl HandlerObject {
    /// Create a new handler with filters
    pub fn new<H, Args, FBox, F>(handler: H, filters: Vec<FBox>) -> Self
    where
        H: Handler<Args> + Clone + Send + Sync + 'static,
        H::Future: Send + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
        FBox: Into<Box<F>>,
        F: Filter + 'static,
    {
        Self {
            service: handler_service(handler),
            filters: Arc::new(
                filters
                    .into_iter()
                    .map(|filter| filter.into() as _)
                    .collect(),
            ),
        }
    }

    /// Create a new handler without filters
    pub fn new_no_filters<H, Args>(handler: H) -> Self
    where
        H: Handler<Args> + Clone + Send + Sync + 'static,
        H::Future: Send + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        Self {
            service: handler_service(handler),
            filters: Arc::new(Vec::new()),
        }
    }

    #[must_use]
    pub fn filters(&self) -> Arc<Vec<Box<dyn Filter>>> {
        Arc::clone(&self.filters)
    }

    /// Register filter for the handler
    /// # Arguments
    /// * `filter` - Filter for the handler
    /// # Panics
    /// If there are other [`Arc`] or `Weak` pointers to the same allocation
    pub fn filter<T, F>(&mut self, filter: T)
    where
        T: Into<Box<F>>,
        F: Filter + 'static,
    {
        Arc::get_mut(&mut self.filters)
            .expect(
                "There are other Arc or Weak pointers to the same allocation. \
            This method can only be called on an exclusive reference. \
            Perhaps you try to register filter in cloned handler?",
            )
            .push(filter.into());
    }
}

impl ServiceFactory<Request> for HandlerObject {
    type Response = Response;
    type Error = app::ErrorKind;
    type Config = ();
    type Service = HandlerObjectService;
    type InitError = ();

    fn new_service(&self, _: Self::Config) -> Result<Self::Service, Self::InitError> {
        let service = self.service.new_service(())?;
        let filters = Arc::clone(&self.filters);

        Ok(HandlerObjectService {
            service: Arc::new(service),
            filters,
        })
    }
}

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

    fn call(&self, req: Request) -> Self::Future {
        self.service.call(req)
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn handler_service<H, Args>(handler: H) -> BoxedHandlerServiceFactory
where
    H: Handler<Args> + Clone + Send + Sync + 'static,
    H::Future: Send + 'static,
    H::Output: Into<EventReturn>,
    Args: FromEventAndContext + 'static,
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
                Err(err) => Err(err.into()),
            }
        }
    }))
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
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
        let filter = command::Command {
            commands: vec![command::PatternType::Text("start")],
            prefix: "/",
            ignore_case: false,
            ignore_mention: false,
        };

        let mut handler_object = HandlerObject::new_no_filters(|| async { unreachable!() });
        assert!(handler_object.filters().is_empty());

        handler_object.filter(filter.clone());
        assert_eq!(handler_object.filters().len(), 1);

        let handler_object = HandlerObject::new(|| async { unreachable!() }, vec![filter]);
        assert_eq!(handler_object.filters().len(), 1);
    }

    #[tokio::test]
    async fn test_handler_object_service() {
        let handler_object = HandlerObject::new_no_filters(|| async {});
        let handler_object_service = handler_object.new_service(()).unwrap();

        let req = Request::new(Bot::default(), Update::default(), Context::new());
        assert!(handler_object_service.check(&req));

        let res = handler_object_service.call(req).await.unwrap();

        assert!(!res.response().is_cancel());
        assert!(!res.response().is_skip());
    }
}
