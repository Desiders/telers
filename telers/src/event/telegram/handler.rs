use super::response::{IntoHandlerResult, Response};
use crate::{
    errors::{ExtractionError, FilterError, HandlerError},
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
    future::Future,
    task::{Context, Poll},
};
use tracing::{event, instrument, Level};

pub(crate) type BoxedCloneHandlerService<Client> =
    BoxCloneService<Request<Client>, Response<Client>, ExtractionError>;

pub trait HandlerFn<Args>: Clone + Send + Sync + 'static {
    type Response: IntoHandlerResult;
    type Future: Future<Output = Self::Response> + Send;

    fn call(&mut self, args: Args) -> Self::Future;
}

pub struct Handler<Client> {
    pub(crate) service: BoxedCloneHandlerService<Client>,
    pub(crate) filters: Vec<BoxedCloneFilterService<Client>>,
}

impl<Client> Handler<Client> {
    #[must_use]
    pub fn new<H, Args>(handler_fn: H) -> Self
    where
        H: HandlerFn<Args>,
        Args: Extractor<Client> + Send,
        Args::Error: Send,
        Client: Send + Sync + 'static,
    {
        Self {
            service: boxed_handler_factory(handler_fn),
            filters: vec![],
        }
    }

    #[must_use]
    pub fn new_service<S, Args>(service: S) -> Self
    where
        S: Service<Args> + Clone + Send + Sync + 'static,
        S::Response: Into<EventReturn>,
        S::Error: Into<anyhow::Error> + Send,
        S::Future: Send,
        Args: Extractor<Client> + Send,
        Args::Error: Send,
        Client: Send + Sync + 'static,
    {
        Self {
            service: boxed_service_factory(service),
            filters: vec![],
        }
    }

    #[must_use]
    pub fn filter(self, val: impl Filter<Client>) -> Self
    where
        Client: Send + Sync + 'static,
    {
        Self {
            service: self.service,
            filters: self
                .filters
                .into_iter()
                .chain(Some(boxed_filter_factory(val)))
                .collect(),
        }
    }
}

impl<Client> Handler<Client>
where
    Client: Send + Sync,
{
    /// Check if the handler pass the filters.
    /// If the handler pass all them, it will be called.
    /// # Errors
    /// If any filter returns error, it will be wrapped into [`FilterError`] and returned
    #[allow(clippy::missing_panics_doc)]
    #[instrument(skip(self, request))]
    pub async fn check(
        &mut self,
        mut request: Request<Client>,
    ) -> Result<(bool, Request<Client>), FilterError> {
        for filter in &mut self.filters {
            let (result, new_request) = filter.call(request).await.map_err(FilterError::new)?;
            if !result {
                return Ok((false, new_request));
            }
            request = new_request;
        }
        Ok((true, request))
    }
}

impl<Client> Clone for Handler<Client> {
    fn clone(&self) -> Self {
        Self {
            service: self.service.clone(),
            filters: self.filters.clone(),
        }
    }
}

impl<Client> Service<Request<Client>> for Handler<Client> {
    type Error = ExtractionError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = Response<Client>;

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
    H: HandlerFn<Args>,
    Args: Extractor<Client> + Send,
    Args::Error: Send,
{
    BoxCloneService::new(service_fn(move |request: Request<Client>| {
        let mut handler = handler.clone();

        async move {
            match Args::extract(&request).await {
                Ok(args) => Ok(Response {
                    request,
                    result: match handler.call(args).await.into_handler_result() {
                        Ok(response) => Ok(response),
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
    S::Error: Into<anyhow::Error> + Send,
    S::Future: Send,
    Args: Extractor<Client> + Send,
    Args::Error: Send,
{
    BoxCloneService::new(service_fn(move |request: Request<Client>| {
        let mut service = service.clone();

        async move {
            match Args::extract(&request).await {
                Ok(args) => Ok(Response {
                    request,
                    result: {
                        if let Err(err) = poll_fn(|cx| service.poll_ready(cx)).await {
                            Err(HandlerError::new(err))
                        } else {
                            match service.call(args).await {
                                Ok(val) => Ok(val.into()),
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
        impl<F, Fut, Response, $($ty,)*> HandlerFn<($($ty,)*)> for F
        where
            F: FnMut($($ty),*) -> Fut + Clone + Send + Sync + 'static,
            Response: IntoHandlerResult,
            Fut: Future<Output = Response> + Send,
        {
            type Response = Response;
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
        types::{ChatPrivate, MessageText, Update, UpdateMessage},
        Bot, Extensions,
    };

    use std::{convert::Infallible, sync::Arc};
    use tokio;

    #[test]
    fn test_handler_composite_filter() {
        let filter = Command::default();

        let handler =
            Handler::<Reqwest>::new(|| async { Ok::<_, Infallible>(EventReturn::Finish) });
        assert!(handler.filters.is_empty());

        let handler = handler.filter(filter.clone());
        assert_eq!(handler.filters.len(), 1);

        let handler =
            Handler::<Reqwest>::new(|| async { Ok::<_, Infallible>(EventReturn::Finish) });
        let handler = handler.filter(filter);
        assert_eq!(handler.filters.len(), 1);
    }

    #[tokio::test]
    async fn test_handler() {
        let mut handler = Handler::new(|(), ()| async { Ok::<_, Infallible>(EventReturn::Finish) });

        let request = Request::<Reqwest> {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(0), ""),
            ))),
            bot: Bot::default(),
            context: crate::Context::default(),
            extensions: Extensions::default(),
        };

        let response = handler.call(request).await.unwrap();

        match response.result {
            Ok(EventReturn::Finish) => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[tokio::test]
    async fn test_service() {
        let mut handler = Handler::new_service(service_fn(|((), ())| async {
            Ok::<_, Infallible>(EventReturn::Finish)
        }));

        let request = Request::<Reqwest> {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(0), ""),
            ))),
            bot: Bot::default(),
            context: crate::Context::default(),
            extensions: Extensions::default(),
        };

        let response = handler.call(request).await.unwrap();

        match response.result {
            Ok(EventReturn::Finish) => {}
            _ => panic!("Unexpected result"),
        }
    }
}
