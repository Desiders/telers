use super::{
    service::{Service, ServiceFactory},
    EventReturn, PropagateEventResult, TelegramHandler, TelegramHandlerObject,
    TelegramHandlerObjectService, TelegramHandlerRequest,
};

use crate::{
    client::Bot, context::Context, error::app, extract::FromEventAndContext, filters::Filter,
    types::Update,
};

use futures_core::future::LocalBoxFuture;
use std::{
    cell::RefCell,
    fmt::{self, Debug, Formatter},
    rc::Rc,
};

/// Data for telegram observer service
#[derive(Clone)]
pub struct Request {
    bot: Rc<Bot>,
    update: Rc<Update>,
    context: Rc<RefCell<Context>>,
}

impl From<Request> for TelegramHandlerRequest {
    fn from(req: Request) -> Self {
        TelegramHandlerRequest::new(req.bot, req.update, req.context)
    }
}

/// Response from telegram observer service
pub struct Response {
    request: Request,
    response: PropagateEventResult,
}

/// Event observer for Telegram events.
/// Here you can register handler with filter.
/// This observer will stop event propagation when first handler is pass.
pub struct Observer {
    /// Event observer name
    event_name: &'static str,
    /// Handlers of the observer
    handlers: Vec<TelegramHandlerObject>,
    /// Common handler of the observer with dummy callback which never will be used
    common_handler: TelegramHandlerObject,
}

impl Observer {
    /// Create a new event observer
    #[must_use]
    pub fn new(event_name: &'static str) -> Self {
        Self {
            event_name,
            handlers: vec![],
            common_handler: TelegramHandlerObject::new(
                || async move { unimplemented!("This is only for filters and without logic") },
                vec![],
            ),
        }
    }

    /// Get event observer name
    #[must_use]
    pub fn event_name(&self) -> &str {
        self.event_name
    }

    /// Get handlers of this event observer
    #[must_use]
    pub fn handlers(&self) -> &[TelegramHandlerObject] {
        &self.handlers
    }

    /// Get filters for all handlers of this event observer
    #[must_use]
    pub fn filters(&self) -> &[Box<dyn Filter>] {
        self.common_handler.filters()
    }

    /// Register filter for all handlers of this event observer
    /// # Arguments
    /// * `filter` - Filter for the observer
    pub fn filter(&mut self, filter: Box<dyn Filter>) {
        self.common_handler.filter(filter);
    }

    /// Register event handler
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `filters` - Filters for the handler
    pub fn register<H, Args>(&mut self, handler: H, filters: Vec<Box<dyn Filter>>)
    where
        H: TelegramHandler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.handlers
            .push(TelegramHandlerObject::new(handler, filters));
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

impl ServiceFactory<Request> for Observer {
    type Response = Response;
    type Error = app::Error;
    type Config = ();
    type Service = ObserverService;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    /// Create [`ObserverService`] from [`Observer`]
    fn new_service(&self, _: Self::Config) -> Self::Future {
        let event_name = self.event_name;
        let futs = self
            .handlers
            .iter()
            .map(|handler| handler.new_service(()))
            .collect::<Vec<_>>();
        let fut = self.common_handler.new_service(());

        Box::pin(async move {
            let mut handlers = vec![];
            for fut in futs {
                handlers.push(fut.await?);
            }

            let common_handler = fut.await?;

            Ok(ObserverService {
                event_name,
                handlers: Rc::new(handlers),
                common_handler: Rc::new(common_handler),
            })
        })
    }
}

/// Service for [`Observer`]
#[allow(clippy::module_name_repetitions)]
pub struct ObserverService {
    /// Event observer name
    event_name: &'static str,
    /// Handler services of the observer
    handlers: Rc<Vec<TelegramHandlerObjectService>>,
    /// Common handler service of the observer with dummy callback which never will be used
    common_handler: Rc<TelegramHandlerObjectService>,
}

impl ObserverService {
    /// Get event observer name
    #[must_use]
    pub fn event_name(&self) -> &str {
        self.event_name
    }

    /// Propagate event to handlers and stops propagation on first match.
    /// Handler will be called when all its filters is pass
    /// # Errors
    /// If handler service returns error.
    /// Probably it's error to extract args to the handler.
    pub async fn trigger(&self, req: Request) -> Result<Response, app::Error> {
        Self::trigger_without_self(
            Rc::clone(&self.handlers),
            Rc::clone(&self.common_handler),
            req,
        )
        .await
    }

    /// We need this method to possible call without [`ObserverService`] lifetime
    #[allow(clippy::similar_names)]
    async fn trigger_without_self(
        handlers: Rc<Vec<TelegramHandlerObjectService>>,
        common_handler: Rc<TelegramHandlerObjectService>,
        req: Request,
    ) -> Result<Response, app::Error> {
        let handler_req = req.clone().into();

        if !common_handler.check(&handler_req) {
            return Ok(Response {
                request: req,
                response: PropagateEventResult::Rejected,
            });
        }

        for handler in handlers.iter() {
            if !handler.check(&handler_req) {
                continue;
            }

            let res = handler.call(handler_req.clone()).await?;
            if res.response().is_skip() {
                continue;
            }
            return Ok(Response {
                request: req,
                response: PropagateEventResult::Handled(res),
            });
        }

        Ok(Response {
            request: req,
            response: PropagateEventResult::Unhandled,
        })
    }
}

impl Debug for ObserverService {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ObserverService")
            .field("event_name", &self.event_name)
            .finish()
    }
}

impl Service<Request> for ObserverService {
    type Response = Response;
    type Error = app::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        Box::pin(Self::trigger_without_self(
            Rc::clone(&self.handlers),
            Rc::clone(&self.common_handler),
            req,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        dispatcher::event::bases::Action,
        filters::{Command, CommandPatternType},
        types::Message,
    };

    macro_rules! r#await {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_observer_trigger() {
        let bot = Rc::new(Bot::default());
        let context = Rc::new(RefCell::new(Context::new()));

        let mut observer = Observer::new("test");
        // Register common filter, which handlers can't pass
        observer.filter(Box::new(Command {
            commands: vec![CommandPatternType::Text("start")],
            prefix: "/",
            ignore_case: false,
            ignore_mention: false,
        }));
        observer.register(|| async { Action::Cancel }, vec![]);
        observer.register(
            || async {
                unimplemented!("It's shouldn't trigger because the first handler handles the event")
            },
            vec![],
        );

        let observer_service = r#await!(observer.new_service(())).unwrap();
        let req = Request {
            bot: Rc::clone(&bot),
            update: Rc::new(Update::default()),
            context: Rc::clone(&context),
        };
        let res = r#await!(observer_service.trigger(req)).unwrap();

        match res.response {
            // Observer has filter, which handlers can't pass, so it will be rejected
            PropagateEventResult::Rejected => {}
            _ => panic!("Unexpected result"),
        }

        let res = r#await!(observer_service.trigger(Request {
            bot: Rc::clone(&bot),
            update: Rc::new(Update {
                message: Some(Message {
                    text: Some("/start".to_string()),
                    ..Message::default()
                }),
                ..Update::default()
            }),
            context: Rc::clone(&context),
        }))
        .unwrap();

        match res.response {
            // Observer has filter, which handlers can pass, so it will be handled
            PropagateEventResult::Handled(_) => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_observer_event_return() {
        async fn handler_first() -> impl Into<EventReturn> {
            Action::Skip
        }

        async fn handler_second() -> impl Into<EventReturn> {
            Action::Cancel
        }

        let bot = Rc::new(Bot::default());
        let context = Rc::new(RefCell::new(Context::new()));
        let update = Rc::new(Update::default());

        let mut observer = Observer::new("test");
        observer.register(handler_first, vec![]);
        observer.register(handler_second, vec![]);

        let observer_service = r#await!(observer.new_service(())).unwrap();

        let res = r#await!(observer_service.trigger(Request {
            bot: Rc::clone(&bot),
            update: Rc::clone(&update),
            context: Rc::clone(&context),
        }))
        .unwrap();

        match res.response {
            PropagateEventResult::Handled(handler_res) => {
                assert_eq!(*handler_res.response(), Action::Cancel.into());
            }
            _ => panic!("Unexpected result"),
        }
    }
}
