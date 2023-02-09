use crate::{
    client::Bot,
    context::Context,
    dispatcher::{
        event::{
            bases::{EventReturn, PropagateEventResult},
            service::{Service as _, ServiceFactory as _, ServiceProvider, ToServiceProvider},
            telegram::handler::{
                Handler, HandlerObject, HandlerObjectService, Request as HandlerRequest,
                Result as HandlerResult,
            },
        },
        middlewares::{
            inner::{
                base::Middlewares as InnerMiddlewares, manager::Manager as InnerMiddlewareManager,
            },
            outer::{
                base::Middlewares as OuterMiddlewares, manager::Manager as OuterMiddlewareManager,
            },
        },
    },
    error::AppErrorKind,
    extract::FromEventAndContext,
    filters::Filter,
    types::Update,
};

use std::{
    fmt::{self, Debug, Formatter},
    sync::Arc,
};

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

impl From<Request> for HandlerRequest {
    fn from(req: Request) -> Self {
        Self::new(req.bot, req.update, req.context)
    }
}

#[derive(Debug)]
pub struct Response {
    pub request: Request,
    pub propagate_result: PropagateEventResult,
}

/// Event observer for telegram events
pub struct Observer {
    pub event_name: &'static str,
    pub handlers: Vec<HandlerObject>,
    pub inner_middlewares: InnerMiddlewareManager,
    pub outer_middlewares: OuterMiddlewareManager,

    /// Handler, which never will be called, but used for common filters for all handlers in the observer
    common: HandlerObject,
}

impl Observer {
    /// Create a new event observer
    /// # Arguments
    /// * `event_name` - Event observer name, can be used for logging
    #[allow(unreachable_code)]
    #[must_use]
    pub fn new(event_name: &'static str) -> Self {
        Self {
            event_name,
            handlers: vec![],
            common: HandlerObject::new_no_filters(|| async move {
                // This handler never will be called, so we can use `unreachable!` macro
                ({
                    unreachable!("This handler never will be used");
                }) as Result<_, _>
            }),
            inner_middlewares: InnerMiddlewareManager::default(),
            outer_middlewares: OuterMiddlewareManager::default(),
        }
    }

    /// Register filter for all handlers in the observer
    pub fn filter<F>(&mut self, filter: F)
    where
        F: Filter + 'static,
    {
        self.common.filter(filter);
    }

    /// Register filters for all handlers in the observer
    pub fn filters<F>(&mut self, filters: Vec<F>)
    where
        F: Filter + 'static,
    {
        self.common.filters(filters);
    }

    /// Register handler with filters
    /// # Arguments
    /// * `handler` - [`Handler`] for the observer
    /// * `filters` - [`Filter`]s for the handler
    pub fn register<H, Args, F>(&mut self, handler: H, filters: Vec<F>)
    where
        H: Handler<Args> + Clone + Send + Sync + 'static,
        H::Future: Send,
        H::Output: Into<HandlerResult>,
        Args: FromEventAndContext + Send,
        Args::Error: Send,
        F: Filter + 'static,
    {
        self.handlers.push(HandlerObject::new(handler, filters));
    }

    /// Register handler without filters
    /// # Arguments
    /// * `handler` - [`Handler`] for the observer
    pub fn register_no_filters<H, Args>(&mut self, handler: H)
    where
        H: Handler<Args> + Clone + Send + Sync + 'static,
        H::Future: Send,
        H::Output: Into<HandlerResult>,
        Args: FromEventAndContext + Send,
        Args::Error: Send,
    {
        self.handlers.push(HandlerObject::new_no_filters(handler));
    }

    /// Register handler with filters
    /// # Notes
    /// This method is alias to [`Observer::register`] method
    /// # Arguments
    /// * `handler` - [`Handler`] for the observer
    /// * `filters` - [`Filter`]s for the handler
    pub fn on<H, Args, F>(&mut self, handler: H, filters: Vec<F>)
    where
        H: Handler<Args> + Clone + Send + Sync + 'static,
        H::Future: Send,
        H::Output: Into<HandlerResult>,
        Args: FromEventAndContext + Send,
        Args::Error: Send,
        F: Filter + 'static,
    {
        self.register(handler, filters);
    }

    /// Register handler without filters
    /// # Notes
    /// This method is alias to [`Observer::register_no_filters`] method
    /// # Arguments
    /// * `handler` - [`Handler`] for the observer
    pub fn on_no_filters<H, Args>(&mut self, handler: H)
    where
        H: Handler<Args> + Clone + Send + Sync + 'static,
        H::Future: Send,
        H::Output: Into<HandlerResult>,
        Args: FromEventAndContext + Send,
        Args::Error: Send,
    {
        self.register_no_filters(handler);
    }
}

impl Debug for Observer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Observer")
            .field("event_name", &self.event_name)
            .finish()
    }
}

impl Default for Observer {
    #[must_use]
    fn default() -> Self {
        Self::new("default")
    }
}

impl AsRef<Observer> for Observer {
    #[must_use]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl ToServiceProvider for Observer {
    type Config = ();
    type ServiceProvider = ObserverInner;
    type InitError = ();

    fn to_service_provider(
        self,
        config: Self::Config,
    ) -> Result<Self::ServiceProvider, Self::InitError> {
        let event_name = self.event_name;
        let handlers = self
            .handlers
            .iter()
            .map(|handler| handler.new_service(config))
            .collect::<Result<Vec<_>, _>>()?;
        let common = self.common.new_service(config)?;
        let inner_middlewares = self.inner_middlewares.middlewares.clone();
        let outer_middlewares = self.outer_middlewares.middlewares.clone();

        Ok(ObserverInner {
            event_name,
            handlers,
            common,
            inner_middlewares,
            outer_middlewares,
        })
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct ObserverInner {
    event_name: &'static str,
    handlers: Vec<HandlerObjectService>,
    common: HandlerObjectService,
    pub(crate) inner_middlewares: InnerMiddlewares,
    pub(crate) outer_middlewares: OuterMiddlewares,
}

impl ServiceProvider for ObserverInner {}

impl ObserverInner {
    /// Propagate event to handlers and stops propagation on first match.
    /// Handler will be called when all its filters is pass.
    /// # Errors
    /// - If any handler returns error. Probably it's error to extract args to the handler.
    pub async fn trigger(&self, request: Request) -> Result<Response, AppErrorKind> {
        let handler_request = request.clone().into();

        // Check observer filters
        if !self.common.check(&handler_request).await {
            return Ok(Response {
                request,
                propagate_result: PropagateEventResult::Rejected,
            });
        }

        for handler in &self.handlers {
            if !handler.check(&handler_request).await {
                continue;
            }

            let response = if self.inner_middlewares.is_empty() {
                handler.call(handler_request.clone()).await?
            } else {
                let middleware = &self.inner_middlewares[0];
                let next_middlewares = Box::new(self.inner_middlewares.clone().into_iter().skip(1));

                // Call first middleware (it will call next middlewares or handler)
                middleware
                    .call(
                        Arc::clone(&handler.service),
                        handler_request.clone(),
                        next_middlewares,
                    )
                    .await?
            };

            return match response.handler_result {
                Ok(EventReturn::Skip) => {
                    continue;
                }
                Ok(EventReturn::Cancel) => Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Rejected,
                }),
                Ok(EventReturn::Finish) | Err(_) => Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Handled(response),
                }),
            };
        }

        // Return a response if the event unhandled by observer
        Ok(Response {
            request,
            propagate_result: PropagateEventResult::Unhandled,
        })
    }
}

impl Debug for ObserverInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ObserverInner")
            .field("event_name", &self.event_name)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{error::EventError, filters::Command, types::Message};

    use anyhow::anyhow;
    use tokio;

    #[allow(unreachable_code)]
    #[tokio::test]
    async fn test_observer_trigger() {
        let bot = Bot::default();
        let context = Context::default();

        let mut observer = Observer::new("test");
        // Register common filter, which handlers can't pass
        observer.filter(Command::builder().prefix("/").command("start").build());
        observer.register_no_filters(|| async { Ok(EventReturn::Finish) });
        observer.register_no_filters(|| async {
            unreachable!("It's shouldn't trigger because the first handler handles the event");

            Ok(EventReturn::Finish)
        });

        let observer_service = observer.to_service_provider(()).unwrap();
        let request = Request::new(bot, Update::default(), context);
        let response = observer_service.trigger(request.clone()).await.unwrap();

        // Filter not pass, so handler should be rejected
        match response.propagate_result {
            PropagateEventResult::Rejected => {}
            _ => panic!("Unexpected result"),
        }

        let request = Request::new(
            request.bot,
            Update {
                message: Some(Message {
                    text: Some("/start".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            },
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
        let mut observer = Observer::new("test");
        observer.register_no_filters(|| async { Err(EventError::new(anyhow!("test"))) });
        observer.register_no_filters(|| async {
            unreachable!("It's shouldn't trigger because the first handler handles the event");

            Ok(EventReturn::Finish)
        });

        let observer_service = observer.to_service_provider(()).unwrap();
        let request = Request::new(Bot::default(), Update::default(), Context::default());
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
        let bot = Bot::default();
        let context = Context::default();
        let update = Update::default();

        let mut observer = Observer::new("test");
        observer.register_no_filters(|| async { Ok(EventReturn::Skip) });
        observer.register_no_filters(|| async { Ok(EventReturn::Finish) });

        let observer_service = observer.to_service_provider(()).unwrap();

        let request = Request::new(bot, update, context);
        let response = observer_service.trigger(request.clone()).await.unwrap();

        // First handler returns `EventReturn::Skip`, so second handler should be called
        match response.propagate_result {
            PropagateEventResult::Handled(response) => match response.handler_result {
                Ok(EventReturn::Finish) => {}
                _ => panic!("Unexpected result"),
            },
            _ => panic!("Unexpected result"),
        }

        let mut observer = Observer::new("test2");
        observer.register_no_filters(|| async { Ok(EventReturn::Skip) });
        observer.register_no_filters(|| async { Ok(EventReturn::Cancel) });

        let observer_service = observer.to_service_provider(()).unwrap();

        let response = observer_service.trigger(request).await.unwrap();

        // First handler returns `EventReturn::Skip`, so second handler should be called and it returns `EventReturn::Cancel`,
        // so response should be `PropagateEventResult::Rejected`
        match response.propagate_result {
            PropagateEventResult::Rejected => {}
            _ => panic!("Unexpected result"),
        }
    }
}
