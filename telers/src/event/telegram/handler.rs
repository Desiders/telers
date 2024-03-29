use crate::{
    client::Reqwest,
    event::{
        service::{
            factory, fn_service, BoxFuture, BoxService, BoxServiceFactory, Service, ServiceFactory,
        },
        EventReturn,
    },
};

use crate::{
    client::Bot,
    context::Context,
    errors::{ExtractionError, HandlerError},
    extractors::FromEventAndContext,
    filters::Filter,
    types::Update,
};

use std::{
    fmt::{self, Debug, Formatter},
    future::Future,
    result::Result as StdResult,
    sync::Arc,
};
use tracing::{event, instrument, Level};

pub type BoxedHandlerService<Client> =
    BoxService<Request<Client>, Response<Client>, ExtractionError>;
pub type BoxedHandlerServiceFactory<Client> =
    BoxServiceFactory<(), Request<Client>, Response<Client>, ExtractionError, ()>;

pub struct Request<Client = Reqwest> {
    pub bot: Arc<Bot<Client>>,
    pub update: Arc<Update>,
    pub context: Arc<Context>,
}

impl<Client> Request<Client> {
    #[must_use]
    pub fn new(bot: Arc<Bot<Client>>, update: Arc<Update>, context: Arc<Context>) -> Self {
        Self {
            bot,
            update,
            context,
        }
    }
}

impl<Client> Debug for Request<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request")
            .field("bot", &self.bot)
            .field("update", &self.update)
            .field("context", &self.context)
            .finish()
    }
}

impl<Client> PartialEq for Request<Client> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.bot, &other.bot)
            && Arc::ptr_eq(&self.update, &other.update)
            && Arc::ptr_eq(&self.context, &other.context)
    }
}

impl<Client> Clone for Request<Client> {
    fn clone(&self) -> Self {
        Self {
            bot: Arc::clone(&self.bot),
            update: Arc::clone(&self.update),
            context: Arc::clone(&self.context),
        }
    }
}

pub type Result = StdResult<EventReturn, HandlerError>;

pub struct Response<Client = Reqwest> {
    pub request: Request<Client>,
    pub handler_result: Result,
}

impl<Client> Debug for Response<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("request", &self.request)
            .field("handler_result", &self.handler_result)
            .finish()
    }
}

pub trait Handler<Args> {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn call(&self, args: Args) -> Self::Future;
}

#[allow(clippy::module_name_repetitions)]
pub struct HandlerObject<Client> {
    service: BoxedHandlerServiceFactory<Client>,

    pub filters: Vec<Arc<dyn Filter<Client>>>,
}

impl<Client> HandlerObject<Client>
where
    Client: Send + Sync + 'static,
{
    pub fn new<H, Args>(handler: H) -> Self
    where
        H: Handler<Args> + Clone + Send + Sync + 'static,
        H::Future: Send,
        H::Output: Into<Result>,
        Args: FromEventAndContext<Client> + Send,
        Args::Error: Send,
    {
        Self {
            service: handler_service(handler),
            filters: vec![],
        }
    }
}

impl<Client> HandlerObject<Client> {
    pub fn filter<T>(&mut self, val: T) -> &mut Self
    where
        T: Filter<Client> + 'static,
    {
        self.filters.push(Arc::new(val));
        self
    }

    pub fn filters<T, I>(&mut self, val: I) -> &mut Self
    where
        T: Filter<Client> + 'static,
        I: IntoIterator<Item = T>,
    {
        self.filters
            .extend(val.into_iter().map(|val| Arc::new(val) as _));
        self
    }
}

impl<Client> ServiceFactory<Request<Client>> for HandlerObject<Client> {
    type Response = Response<Client>;
    type Error = ExtractionError;
    type Config = ();
    type Service = HandlerObjectService<Client>;
    type InitError = ();

    fn new_service(&self, config: Self::Config) -> StdResult<Self::Service, Self::InitError> {
        let service = self.service.new_service(config)?;

        Ok(HandlerObjectService {
            service: Arc::new(service),
            filters: self.filters.clone().into(),
        })
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct HandlerObjectService<Client> {
    pub(crate) service: Arc<BoxedHandlerService<Client>>,
    filters: Box<[Arc<dyn Filter<Client>>]>,
}

impl<Client> HandlerObjectService<Client>
where
    Client: Sync,
{
    /// Check if the handler pass the filters.
    /// If the handler pass all them, it will be called.
    #[instrument(skip(self, request))]
    pub async fn check(&self, request: &Request<Client>) -> bool {
        for filter in &*self.filters {
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

impl<Client> Service<Request<Client>> for HandlerObjectService<Client> {
    type Response = Response<Client>;
    type Error = ExtractionError;
    type Future = BoxFuture<StdResult<Self::Response, Self::Error>>;

    fn call(&self, req: Request<Client>) -> Self::Future {
        self.service.call(req)
    }
}

#[allow(clippy::module_name_repetitions)]
#[instrument(skip(handler))]
pub fn handler_service<Client, H, Args>(handler: H) -> BoxedHandlerServiceFactory<Client>
where
    Client: Send + Sync + 'static,
    H: Handler<Args> + Clone + Send + Sync + 'static,
    H::Future: Send,
    H::Output: Into<Result>,
    Args: FromEventAndContext<Client> + Send,
    Args::Error: Send,
{
    factory(fn_service(move |request: Request<Client>| {
        let bot = Arc::clone(&request.bot);
        let update = Arc::clone(&request.update);
        let context = Arc::clone(&request.context);

        let handler = handler.clone();

        async move {
            match Args::extract(bot, update, context) {
                Ok(extracted_args) => Ok(Response {
                    request,
                    handler_result: handler.call(extracted_args).await.into(),
                }),
                Err(extraction_err) => {
                    let extraction_err = extraction_err.into();

                    event!(
                        Level::ERROR,
                        error = %extraction_err,
                        bot = ?request.bot,
                        update = ?request.update,
                        context = ?request.context,
                        "Failed to extract arguments",
                    );

                    Err(extraction_err)
                }
            }
        }
    }))
}

#[allow(non_snake_case)]
mod factory_handlers {
    //! This module is used to implement [`Handler`] for function-like with 0-20 arguments

    use super::{Future, Handler};

    macro_rules! factory ({ $($param:ident)* } => {
        impl<Func, Fut, $($param,)*> Handler<($($param,)*)> for Func
        where
            Func: Fn($($param,)*) -> Fut,
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
    // To be able to extract tuple with 13 arguments
    factory! { A B C D E F G H I J K L M}
    // To be able to extract tuple with 14 arguments
    factory! { A B C D E F G H I J K L M N }
    // To be able to extract tuple with 15 arguments
    factory! { A B C D E F G H I J K L M N O}
    // To be able to extract tuple with 16 arguments
    factory! { A B C D E F G H I J K L M N O P }
    // To be able to extract tuple with 17 arguments
    factory! { A B C D E F G H I J K L M N O P Q }
    // To be able to extract tuple with 18 arguments
    factory! { A B C D E F G H I J K L M N O P Q R }
    // To be able to extract tuple with 19 arguments
    factory! { A B C D E F G H I J K L M N O P Q R S }
    // To be able to extract tuple with 20 arguments
    factory! { A B C D E F G H I J K L M N O P Q R S T }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Reqwest,
        event::EventReturn,
        filters::Command,
        types::{Message, UpdateKind},
    };

    use tokio;

    #[test]
    fn test_arg_number() {
        fn assert_impl_handler<Client, T: FromEventAndContext<Client>>(_: impl Handler<T>) {}

        assert_impl_handler::<Reqwest, _>(|| async { unreachable!() });
        assert_impl_handler::<Reqwest, _>(
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
             _12: (),
             _13: (),
             _14: (),
             _15: (),
             _16: (),
             _17: (),
             _18: (),
             _19: (),
             _20: ()| async { unreachable!() },
        );
    }

    #[test]
    fn test_handler_object_filter() {
        let filter = Command::default();

        let mut handler_object =
            HandlerObject::<Reqwest>::new(|| async { Ok(EventReturn::Finish) });
        assert!(handler_object.filters.is_empty());

        handler_object.filter(filter.clone());
        assert_eq!(handler_object.filters.len(), 1);

        let mut handler_object =
            HandlerObject::<Reqwest>::new(|| async { Ok(EventReturn::Finish) });
        handler_object.filter(filter);
        assert_eq!(handler_object.filters.len(), 1);
    }

    #[tokio::test]
    async fn test_handler_object_service() {
        let handler_object = HandlerObject::<Reqwest>::new(|| async { Ok(EventReturn::Finish) });
        let handler_object_service = handler_object.new_service(()).unwrap();

        let request = Request::new(
            Arc::new(Bot::<Reqwest>::default()),
            Arc::new(Update {
                id: 0,
                kind: UpdateKind::Message(Message::default()),
            }),
            Arc::new(Context::default()),
        );
        let response = handler_object_service.call(request).await.unwrap();

        match response.handler_result {
            Ok(EventReturn::Finish) => {}
            _ => panic!("Unexpected result"),
        }
    }
}
