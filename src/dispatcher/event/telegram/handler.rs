use crate::{
    client::Bot,
    context::Context,
    dispatcher::event::{
        service::{
            factory, fn_service, BoxFuture, BoxService, BoxServiceFactory, Service, ServiceFactory,
        },
        EventReturn,
    },
    error::{EventError, ExtractionError},
    extract::FromEventAndContext,
    filters::Filter,
    types::Update,
};

use std::{future::Future, result::Result as StdResult, sync::Arc};

pub type BoxedHandlerService = BoxService<Request, Response, ExtractionError>;
pub type BoxedHandlerServiceFactory = BoxServiceFactory<(), Request, Response, ExtractionError, ()>;

/// Request for a handler service
#[derive(Debug, Clone)]
pub struct Request {
    pub bot: Arc<Bot>,
    pub update: Arc<Update>,
    pub context: Arc<Context>,
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
}

impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.bot, &other.bot)
            && Arc::ptr_eq(&self.update, &other.update)
            && Arc::ptr_eq(&self.context, &other.context)
    }
}

pub type Result = StdResult<EventReturn, EventError>;

#[derive(Debug)]
pub struct Response {
    pub request: Request,
    pub handler_result: Result,
}

pub trait Handler<Args> {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn call(&self, args: Args) -> Self::Future;
}

#[allow(clippy::module_name_repetitions)]
pub struct HandlerObject {
    service: BoxedHandlerServiceFactory,
    pub filters: Vec<Arc<Box<dyn Filter>>>,
}

impl HandlerObject {
    /// Create a new handler with filters
    pub fn new<H, Args, F>(handler: H, filters: Vec<F>) -> Self
    where
        H: Handler<Args> + Clone + Send + Sync + 'static,
        H::Future: Send,
        H::Output: Into<Result>,
        Args: FromEventAndContext + Send,
        Args::Error: Send,
        F: Filter + 'static,
    {
        Self {
            service: handler_service(handler),
            filters: filters
                .into_iter()
                .map(|filter| Arc::new(Box::new(filter) as _))
                .collect(),
        }
    }

    /// Create a new handler without filters
    pub fn new_no_filters<H, Args>(handler: H) -> Self
    where
        H: Handler<Args> + Clone + Send + Sync + 'static,
        H::Future: Send,
        H::Output: Into<Result>,
        Args: FromEventAndContext + Send,
        Args::Error: Send,
    {
        Self {
            service: handler_service(handler),
            filters: vec![],
        }
    }

    /// Register filter for the handler
    pub fn filter<F: Filter + 'static>(&mut self, filter: F) {
        self.filters.push(Arc::new(Box::new(filter)));
    }

    /// Register filters for the handler
    pub fn filters<F: Filter + 'static>(&mut self, filters: Vec<F>) {
        self.filters.extend(
            filters
                .into_iter()
                .map(|filter| Arc::new(Box::new(filter) as _)),
        );
    }
}

impl ServiceFactory<Request> for HandlerObject {
    type Response = Response;
    type Error = ExtractionError;
    type Config = ();
    type Service = HandlerObjectService;
    type InitError = ();

    fn new_service(&self, config: Self::Config) -> StdResult<Self::Service, Self::InitError> {
        let service = self.service.new_service(config)?;
        let filters = self.filters.clone();

        Ok(HandlerObjectService {
            service: Arc::new(service),
            filters,
        })
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct HandlerObjectService {
    pub(crate) service: Arc<BoxedHandlerService>,
    filters: Vec<Arc<Box<dyn Filter>>>,
}

impl HandlerObjectService {
    /// Check if the handler pass the filters.
    /// If the handler pass all them, it will be called.
    pub async fn check(&self, request: &Request) -> bool {
        for filter in &self.filters {
            if !filter
                .check(&request.bot, &request.update, &request.context)
                .await
            {
                return false;
            }
        }
        true
    }
}

impl Service<Request> for HandlerObjectService {
    type Response = Response;
    type Error = ExtractionError;
    type Future = BoxFuture<StdResult<Self::Response, Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        self.service.call(req)
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn handler_service<H, Args>(handler: H) -> BoxedHandlerServiceFactory
where
    H: Handler<Args> + Clone + Send + Sync + 'static,
    H::Future: Send,
    H::Output: Into<Result>,
    Args: FromEventAndContext + Send,
    Args::Error: Send,
{
    factory(fn_service(move |request: Request| {
        let handler = handler.clone();

        async move {
            match Args::extract(
                Arc::clone(&request.bot),
                Arc::clone(&request.update),
                Arc::clone(&request.context),
            ) {
                Ok(extracted_args) => Ok(Response {
                    request,
                    handler_result: handler.call(extracted_args).await.into(),
                }),
                Err(extraction_err) => Err(extraction_err.into()),
            }
        }
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
    use crate::{dispatcher::event::EventReturn, filters::Command};

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
        let filter = Command::default();

        let mut handler_object =
            HandlerObject::new_no_filters(|| async { Ok(EventReturn::Finish) });
        assert!(handler_object.filters.is_empty());

        handler_object.filter(filter.clone());
        assert_eq!(handler_object.filters.len(), 1);

        let handler_object = HandlerObject::new(|| async { Ok(EventReturn::Finish) }, vec![filter]);
        assert_eq!(handler_object.filters.len(), 1);
    }

    #[tokio::test]
    async fn test_handler_object_service() {
        let handler_object = HandlerObject::new_no_filters(|| async { Ok(EventReturn::Finish) });
        let handler_object_service = handler_object.new_service(()).unwrap();

        let request = Request::new(Bot::default(), Update::default(), Context::new());
        let response = handler_object_service.call(request).await.unwrap();

        match response.handler_result {
            Ok(EventReturn::Finish) => {}
            _ => panic!("Unexpected result"),
        }
    }
}
