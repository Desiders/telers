use crate::{
    client::Bot,
    context::Context,
    enums::TelegramObserverName,
    errors::EventErrorKind,
    event::{
        bases::{EventReturn, PropagateEventResult},
        service::{Service as _, ServiceFactory as _, ServiceProvider, ToServiceProvider},
        telegram::handler::{
            Handler, HandlerObject, HandlerObjectService, Request as HandlerRequest,
            Result as HandlerResult,
        },
    },
    extractors::FromEventAndContext,
    filters::Filter,
    middlewares::{
        inner::{
            wrap_handler_and_middlewares_to_next, Manager as InnerMiddlewareManager,
            Middleware as InnerMiddleware,
        },
        outer::{Manager as OuterMiddlewareManager, Middleware as OuterMiddleware},
    },
    types::Update,
};

use std::{
    fmt::{self, Debug, Formatter},
    sync::Arc,
};
use tracing::{event, instrument, Level};

pub struct Request<Client> {
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

impl<Client> Clone for Request<Client> {
    fn clone(&self) -> Self {
        Self {
            bot: Arc::clone(&self.bot),
            update: Arc::clone(&self.update),
            context: Arc::clone(&self.context),
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

impl<Client> From<Request<Client>> for HandlerRequest<Client> {
    fn from(req: Request<Client>) -> Self {
        Self::new(req.bot, req.update, req.context)
    }
}

pub struct Response<Client> {
    pub request: Request<Client>,
    pub propagate_result: PropagateEventResult<Client>,
}

impl<Client> Debug for Response<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("request", &self.request)
            .field("propagate_result", &self.propagate_result)
            .finish()
    }
}

/// Event observer for telegram events
pub struct Observer<Client> {
    pub event_name: TelegramObserverName,

    handlers: Vec<HandlerObject<Client>>,
    common: Box<HandlerObject<Client>>,

    pub inner_middlewares: InnerMiddlewareManager<Client>,
    pub outer_middlewares: OuterMiddlewareManager<Client>,
}

impl<Client> Observer<Client> {
    #[allow(unreachable_code)]
    #[must_use]
    pub fn new(event_name: TelegramObserverName) -> Self
    where
        Client: Send + Sync + 'static,
    {
        Self {
            event_name,
            handlers: vec![],
            common: Box::new(HandlerObject::<Client>::new(|| async move {
                // This handler never will be called, so we can use `unreachable!` macro
                ({
                    unreachable!("This handler never will be used");
                }) as Result<_, _>
            })),
            inner_middlewares: InnerMiddlewareManager::<Client>::default(),
            outer_middlewares: OuterMiddlewareManager::<Client>::default(),
        }
    }

    #[must_use]
    pub fn handlers(&self) -> &[HandlerObject<Client>] {
        &self.handlers
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn register<H, Args>(&mut self, handler: H) -> &mut HandlerObject<Client>
    where
        Client: Send + Sync + 'static,
        H: Handler<Args> + Clone + Send + Sync + 'static,
        H::Future: Send,
        H::Output: Into<HandlerResult>,
        Args: FromEventAndContext<Client> + Send,
        Args::Error: Send,
    {
        self.handlers.push(HandlerObject::new(handler));
        // `unwrap` is safe, because we just added element to the vector
        self.handlers.last_mut().unwrap()
    }

    /// Alias to [`Observer::register`] method
    pub fn on<H, Args>(&mut self, handler: H) -> &mut HandlerObject<Client>
    where
        Client: Send + Sync + 'static,
        H: Handler<Args> + Clone + Send + Sync + 'static,
        H::Future: Send,
        H::Output: Into<HandlerResult>,
        Args: FromEventAndContext<Client> + Send,
        Args::Error: Send,
    {
        self.register(handler)
    }

    /// Register filter for all handlers in the observer
    pub fn filter<T>(&mut self, val: T) -> &mut Self
    where
        T: Filter<Client> + 'static,
    {
        self.common.filter(val);
        self
    }

    /// Register filters for all handlers in the observer
    pub fn filters<T, I>(&mut self, val: I) -> &mut Self
    where
        T: Filter<Client> + 'static,
        I: IntoIterator<Item = T>,
    {
        self.common.filters(val);
        self
    }
}

impl<Client> Debug for Observer<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Observer")
            .field("event_name", &self.event_name)
            .finish_non_exhaustive()
    }
}

impl<Client> Default for Observer<Client>
where
    Client: Send + Sync + 'static,
{
    #[must_use]
    fn default() -> Self {
        Self::new(TelegramObserverName::Message)
    }
}

impl<Client> AsRef<Observer<Client>> for Observer<Client> {
    #[must_use]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<Client> ToServiceProvider for Observer<Client> {
    type Config = ();
    type ServiceProvider = Service<Client>;
    type InitError = ();

    fn to_service_provider(
        self,
        config: Self::Config,
    ) -> Result<Self::ServiceProvider, Self::InitError> {
        Ok(Service {
            event_name: self.event_name,
            handlers: self
                .handlers
                .iter()
                .map(|handler| handler.new_service(config))
                .collect::<Result<_, _>>()?,
            common: self.common.new_service(config)?,
            inner_middlewares: self.inner_middlewares.middlewares.into(),
            outer_middlewares: self.outer_middlewares.middlewares.into(),
        })
    }
}

pub struct Service<Client> {
    pub(crate) event_name: TelegramObserverName,

    handlers: Box<[HandlerObjectService<Client>]>,
    common: HandlerObjectService<Client>,

    inner_middlewares: Box<[Arc<dyn InnerMiddleware<Client>>]>,
    outer_middlewares: Box<[Arc<dyn OuterMiddleware<Client>>]>,
}

impl<Client> ServiceProvider for Service<Client> {}

impl<Client> Service<Client> {
    /// Propagate event to handlers and stops propagation on first match.
    /// Handler will be called when all its filters is pass.
    /// # Errors
    /// - If any handler returns error. Probably it's error to extract args to the handler.
    #[instrument(skip(self, request))]
    pub async fn trigger(
        &self,
        request: Request<Client>,
    ) -> Result<Response<Client>, EventErrorKind>
    where
        Client: Send + Sync + 'static,
    {
        let handler_request: HandlerRequest<Client> = request.clone().into();

        // Check observer filters
        if !self.common.check(&handler_request).await {
            event!(Level::TRACE, "Request are not pass observer filters");

            return Ok(Response {
                request,
                propagate_result: PropagateEventResult::Rejected,
            });
        }

        // Check handlers filters
        for handler in &*self.handlers {
            if !handler.check(&handler_request).await {
                continue;
            }

            event!(Level::TRACE, "Request are pass handler filters");

            let response = match self.inner_middlewares.split_first() {
                Some((middleware, middlewares)) => {
                    let next = Box::new(wrap_handler_and_middlewares_to_next(
                        Arc::clone(&handler.service),
                        middlewares.to_vec().into_boxed_slice(), // we use it instead of `into` because some versions of rustc can't infer type
                    ));
                    middleware.call(handler_request.clone(), next).await
                }
                None => handler
                    .call(handler_request.clone())
                    .await
                    .map_err(EventErrorKind::Extraction),
            }?;

            return match response.handler_result {
                // If the handler or middleware returns skip, then we should skip it
                Ok(EventReturn::Skip) => {
                    event!(Level::TRACE, "Handler returns skip");

                    continue;
                }
                // If the handler or middleware returns cancel, then we should stop propagation
                Ok(EventReturn::Cancel) => {
                    event!(Level::TRACE, "Handler returns cancel");

                    Ok(Response {
                        request,
                        propagate_result: PropagateEventResult::Rejected,
                    })
                }
                // If the handler or middleware returns finish, then we should stop propagation and return a response
                Ok(EventReturn::Finish) => {
                    event!(Level::TRACE, "Handler returns finish");

                    Ok(Response {
                        request,
                        propagate_result: PropagateEventResult::Handled(response),
                    })
                }
                // If the handler or middleware returns an error,
                // then we should stop propagation and return a response
                // because the error is the correct result from the point of view of observer
                Err(_) => {
                    event!(Level::TRACE, "Handler returns error");

                    Ok(Response {
                        request,
                        propagate_result: PropagateEventResult::Handled(response),
                    })
                }
            };
        }

        event!(Level::TRACE, "Request are not pass handlers filters");

        // If all handlers are not pass filters, then we should call common handler
        Ok(Response {
            request,
            propagate_result: PropagateEventResult::Unhandled,
        })
    }

    #[must_use]
    pub fn inner_middlewares(&self) -> &[Arc<dyn InnerMiddleware<Client>>] {
        &self.inner_middlewares
    }

    #[must_use]
    pub fn outer_middlewares(&self) -> &[Arc<dyn OuterMiddleware<Client>>] {
        &self.outer_middlewares
    }
}

impl<Client> Debug for Service<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Service")
            .field("event_name", &self.event_name)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Reqwest,
        errors::HandlerError,
        filters::Command,
        types::{Message, MessageText, UpdateKind},
    };

    use anyhow::anyhow;
    use tokio;

    #[allow(unreachable_code)]
    #[tokio::test]
    async fn test_observer_trigger() {
        let mut observer = Observer::default();
        // Register common filter, which handlers can't pass
        observer.filter(Command::one("start"));
        observer.register(|| async { Ok(EventReturn::Finish) });
        observer.register(|| async {
            unreachable!("It's shouldn't trigger because the first handler handles the event");

            Ok(EventReturn::Finish)
        });

        let observer_service = observer.to_service_provider_default().unwrap();
        let request = Request::new(
            Arc::new(Bot::<Reqwest>::default()),
            Arc::new(Update::default()),
            Arc::new(Context::default()),
        );
        let response = observer_service.trigger(request.clone()).await.unwrap();

        // Filter not pass, so handler should be rejected
        match response.propagate_result {
            PropagateEventResult::Rejected => {}
            _ => panic!("Unexpected result"),
        }

        let request = Request::new(
            request.bot,
            Arc::new(Update {
                kind: UpdateKind::Message(Message::Text(Box::new(MessageText {
                    text: "/start".to_owned().into(),
                    ..Default::default()
                }))),
                ..Default::default()
            }),
            request.context,
        );
        let response = observer_service.trigger(request).await.unwrap();

        // Filter pass, so handler should be handled
        match response.propagate_result {
            PropagateEventResult::Handled(_) => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[allow(unreachable_code)]
    #[tokio::test]
    async fn test_observer_trigger_error() {
        let mut observer = Observer::<Reqwest>::default();
        observer.register(|| async { Err(HandlerError::new(anyhow!("test"))) });
        observer.register(|| async {
            unreachable!("It's shouldn't trigger because the first handler handles the event");

            Ok(EventReturn::Finish)
        });

        let observer_service = observer.to_service_provider_default().unwrap();
        let request = Request::new(
            Arc::new(Bot::default()),
            Arc::new(Update::default()),
            Arc::new(Context::default()),
        );
        let response = observer_service.trigger(request).await.unwrap();

        // First handler returns error, second handler shouldn't be called
        match response.propagate_result {
            PropagateEventResult::Handled(response) => match response.handler_result {
                Err(_) => {}
                _ => panic!("Unexpected result"),
            },
            _ => panic!("Unexpected result"),
        }
    }

    #[tokio::test]
    async fn test_observer_event_return() {
        let mut observer = Observer::default();
        observer.register(|| async { Ok(EventReturn::Skip) });
        observer.register(|| async { Ok(EventReturn::Finish) });

        let observer_service = observer.to_service_provider_default().unwrap();

        let request = Request::new(
            Arc::new(Bot::<Reqwest>::default()),
            Arc::new(Update::default()),
            Arc::new(Context::default()),
        );
        let response = observer_service.trigger(request.clone()).await.unwrap();

        // First handler returns `EventReturn::Skip`, so second handler should be called
        match response.propagate_result {
            PropagateEventResult::Handled(response) => match response.handler_result {
                Ok(EventReturn::Finish) => {}
                _ => panic!("Unexpected result"),
            },
            _ => panic!("Unexpected result"),
        }

        let mut observer = Observer::default();
        observer.register(|| async { Ok(EventReturn::Skip) });
        observer.register(|| async { Ok(EventReturn::Cancel) });

        let observer_service = observer.to_service_provider_default().unwrap();

        let response = observer_service.trigger(request).await.unwrap();

        // First handler returns `EventReturn::Skip`, so second handler should be called and it returns `EventReturn::Cancel`,
        // so response should be `PropagateEventResult::Rejected`
        match response.propagate_result {
            PropagateEventResult::Rejected => {}
            _ => panic!("Unexpected result"),
        }
    }
}
