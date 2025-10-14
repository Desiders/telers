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

use futures_util::future::{poll_fn, BoxFuture};
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
    type Response: Into<EventReturn>;
    type Error: Into<anyhow::Error>;
    type Future: Future<Output = Result<Self::Response, Self::Error>> + Send;

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

    pub fn new_service<S, Args>(service: S) -> Self
    where
        S: Service<Args> + Clone + Send + Sync + 'static,
        S::Response: Into<EventReturn>,
        S::Error: Into<anyhow::Error> + Send + Sync + 'static,
        S::Future: Send,
        Args: Extractor<Client> + Send,
        Args::Error: Send,
    {
        Self {
            service: boxed_service_factory(service),
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
                Ok(args) => Ok(Response {
                    request,
                    handler_result: match handler.call(args).await {
                        Ok(response) => Ok(response.into()),
                        Err(err) => Err(HandlerError::new(err)),
                    },
                }),
                Err(err) => {
                    let err = err.into();

                    event!(
                        Level::ERROR,
                        error = %err,
                        ?request,
                        "Failed to extract arguments",
                    );

                    Err(err)
                }
            }
        }
    }))
}

pub(crate) fn boxed_service_factory<Client, S, Args>(service: S) -> BoxedCloneHandlerService<Client>
where
    Client: Send + Sync + 'static,
    S: Service<Args> + Clone + Send + Sync + 'static,
    S::Response: Into<EventReturn>,
    S::Error: Into<anyhow::Error> + Send + Sync + 'static,
    S::Future: Send,
    Args: Extractor<Client> + Send,
    Args::Error: Send,
{
    BoxCloneService::new(service_fn(move |request: Request<Client>| {
        let mut service = service.clone();

        async move {
            match Args::extract(&request) {
                Ok(args) => Ok(Response {
                    request,
                    handler_result: {
                        if let Err(err) = poll_fn(|cx| service.poll_ready(cx)).await {
                            Err(HandlerError::new(err))
                        } else {
                            match service.call(args).await {
                                Ok(response) => Ok(response.into()),
                                Err(err) => Err(HandlerError::new(err)),
                            }
                        }
                    },
                }),
                Err(err) => {
                    let err = err.into();

                    event!(
                        Level::ERROR,
                        error = %err,
                        ?request,
                        "Failed to extract arguments",
                    );

                    Err(err)
                }
            }
        }
    }))
}

macro_rules! impl_handlers {
    (
        [$($ty:ident),*]
    ) => {
        impl<F, Fut, Response, Err, $($ty,)*> Handler<($($ty,)*)> for F
        where
            F: FnMut($($ty),*) -> Fut + Clone + Send + Sync + 'static,
            Response: Into<EventReturn>,
            Err: Into<anyhow::Error>,
            Fut: Future<Output = Result<Response, Err>> + Send,
        {
            type Response = Response;
            type Error = Err;
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

    use std::{convert::Infallible, sync::Arc};
    use tokio;

    #[test]
    fn test_handler_composite_filter() {
        let filter = Command::default();

        let mut handler =
            HandlerComposite::<Reqwest>::new(|| async { Ok::<_, Infallible>(EventReturn::Finish) });
        assert!(handler.filters.is_empty());

        handler.filter(filter.clone());
        assert_eq!(handler.filters.len(), 1);

        let mut handler =
            HandlerComposite::<Reqwest>::new(|| async { Ok::<_, Infallible>(EventReturn::Finish) });
        handler.filter(filter);
        assert_eq!(handler.filters.len(), 1);
    }

    #[tokio::test]
    async fn test_handler() {
        let mut handler =
            HandlerComposite::new(|(), ()| async { Ok::<_, Infallible>(EventReturn::Finish) });

        let request = Request::<Reqwest> {
            update: Arc::new(Update {
                id: 0,
                kind: UpdateKind::Message(Message::default()),
            }),
            ..Default::default()
        };

        let response = handler.call(request).await.unwrap();

        match response.handler_result {
            Ok(EventReturn::Finish) => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[tokio::test]
    async fn test_service() {
        let mut handler = HandlerComposite::new_service(service_fn(|((), ())| async {
            Ok::<_, Infallible>(EventReturn::Finish)
        }));

        let request = Request::<Reqwest> {
            update: Arc::new(Update {
                id: 0,
                kind: UpdateKind::Message(Message::default()),
            }),
            ..Default::default()
        };

        let response = handler.call(request).await.unwrap();

        match response.handler_result {
            Ok(EventReturn::Finish) => {}
            _ => panic!("Unexpected result"),
        }
    }
}
