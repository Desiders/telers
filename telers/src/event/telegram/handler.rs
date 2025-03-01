use crate::{
    client::Reqwest,
    errors::{ExtractionError, HandlerError},
    event::{
        service::{service_fn, BoxCloneService, Service},
        EventReturn,
    },
    extractor::Extractor,
    filters::{
        base::{boxed_filter_factory, BoxedCloneFilterService},
        Filter,
    },
    Request,
};

use futures_util::future::BoxFuture;
use std::{
    fmt::{self, Debug, Formatter},
    future::Future,
    task::{Context, Poll},
};
use tracing::{event, instrument, Level};

pub(crate) type BoxedCloneHandlerService<Client> =
    BoxCloneService<Request<Client>, Response<Client>, ExtractionError>;

pub type HandlerResult = Result<EventReturn, HandlerError>;

pub struct Response<Client = Reqwest> {
    pub request: Request<Client>,
    pub handler_result: HandlerResult,
}

impl<Client> Debug for Response<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("request", &self.request)
            .field("handler_result", &self.handler_result)
            .finish()
    }
}

pub trait Handler<Args>: Clone + Send + Sync + 'static {
    type Output: Into<HandlerResult>;
    type Future: Future<Output = Self::Output> + Send;

    fn call(&mut self, args: Args) -> Self::Future;
}

pub struct HandlerComposite<Client> {
    pub(crate) service: BoxedCloneHandlerService<Client>,
    pub(crate) filters: Vec<BoxedCloneFilterService<Client>>,
}

impl<Client> HandlerComposite<Client>
where
    Client: Send + Sync + 'static,
{
    pub fn new<H, Args>(handler: H) -> Self
    where
        H: Handler<Args>,
        Args: Extractor<Client> + Send,
        Args::Error: Send,
    {
        Self {
            service: boxed_handler_factory(handler),
            filters: vec![],
        }
    }

    /// Register filter for current handler
    pub fn filter<F>(&mut self, val: F) -> &mut Self
    where
        F: Filter<Client>,
    {
        self.filters.push(boxed_filter_factory(val));
        self
    }

    /// Register filters for current handler
    pub fn filters<F, I>(&mut self, val: I) -> &mut Self
    where
        F: Filter<Client>,
        I: IntoIterator<Item = F>,
    {
        self.filters
            .extend(val.into_iter().map(boxed_filter_factory));
        self
    }
}

impl<Client> HandlerComposite<Client>
where
    Client: Send + Sync,
{
    /// Check if the handler pass the filters.
    /// If the handler pass all them, it will be called.
    #[instrument(skip(self, request))]
    pub async fn check(&mut self, mut request: Request<Client>) -> (bool, Request<Client>) {
        for filter in &mut self.filters {
            let (result, new_request) = filter.call(request).await.unwrap();
            if !result {
                return (false, new_request);
            }
            request = new_request;
        }
        (true, request)
    }
}

impl<Client> Clone for HandlerComposite<Client> {
    fn clone(&self) -> Self {
        Self {
            service: self.service.clone(),
            filters: self.filters.clone(),
        }
    }
}

impl<Client> Service<Request<Client>> for HandlerComposite<Client> {
    type Response = Response<Client>;
    type Error = ExtractionError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Client>) -> Self::Future {
        self.service.call(req)
    }
}

pub(crate) fn boxed_handler_factory<Client, H, Args>(handler: H) -> BoxedCloneHandlerService<Client>
where
    Client: Send + Sync + 'static,
    H: Handler<Args>,
    Args: Extractor<Client> + Send,
    Args::Error: Send,
{
    BoxCloneService::new(service_fn(move |request: Request<Client>| {
        let mut handler = handler.clone();

        async move {
            match Args::extract(&request) {
                Ok(extracted_args) => Ok(Response {
                    request,
                    handler_result: handler.call(extracted_args).await.into(),
                }),
                Err(extraction_err) => {
                    let extraction_err = extraction_err.into();

                    event!(
                        Level::ERROR,
                        error = %extraction_err,
                        ?request,
                        "Failed to extract arguments",
                    );

                    Err(extraction_err)
                }
            }
        }
    }))
}

// `Handler` implementation for function-like
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Reqwest,
        event::EventReturn,
        filters::Command,
        types::{Message, Update, UpdateKind},
    };

    use std::sync::Arc;
    use tokio;

    #[test]
    fn test_handler_composite_filter() {
        let filter = Command::default();

        let mut handler = HandlerComposite::<Reqwest>::new(|| async { Ok(EventReturn::Finish) });
        assert!(handler.filters.is_empty());

        handler.filter(filter.clone());
        assert_eq!(handler.filters.len(), 1);

        let mut handler = HandlerComposite::<Reqwest>::new(|| async { Ok(EventReturn::Finish) });
        handler.filter(filter);
        assert_eq!(handler.filters.len(), 1);
    }

    #[tokio::test]
    async fn test_handler_service() {
        let mut handler = HandlerComposite::<Reqwest>::new(|| async { Ok(EventReturn::Finish) });

        let mut request = Request::<Reqwest>::default();
        request.update = Arc::new(Update {
            id: 0,
            kind: UpdateKind::Message(Message::default()),
        });

        let response = handler.call(request).await.unwrap();

        match response.handler_result {
            Ok(EventReturn::Finish) => {}
            _ => panic!("Unexpected result"),
        }
    }
}
