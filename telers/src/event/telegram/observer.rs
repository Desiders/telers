use crate::{
    errors::EventErrorKind,
    event::{
        bases::{EventReturn, PropagateEventResult},
        service::Service,
        telegram::handler::Handler,
    },
    filters::Filter,
    middlewares::{
        inner::{
            wrap_to_next, BoxedCloneMiddlewareService as BoxedCloneInnerMiddlewareService,
            Manager as InnerMiddlewareManager,
        },
        outer::{
            BoxedCloneMiddlewareService as BoxedCloneOuterMiddlewareService,
            Manager as OuterMiddlewareManager,
        },
    },
    Request,
};

use std::{
    convert::Infallible,
    fmt::{self, Debug, Formatter},
};
use tracing::{event, instrument, Level};

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
    pub event_name: &'static str,

    handlers: Vec<Handler<Client>>,
    common: Option<Handler<Client>>,

    pub inner_middlewares: InnerMiddlewareManager<Client>,
    pub outer_middlewares: OuterMiddlewareManager<Client>,
}

impl<Client> Observer<Client>
where
    Client: Send + Sync + 'static,
{
    #[allow(unreachable_code)]
    #[must_use]
    pub fn new(event_name: &'static str) -> Self {
        Self {
            event_name,
            handlers: vec![],
            common: None,
            inner_middlewares: InnerMiddlewareManager::<Client>::default(),
            outer_middlewares: OuterMiddlewareManager::<Client>::default(),
        }
    }

    /// Register event handler
    pub fn register(&mut self, handler_fn: Handler<Client>) -> &mut Self {
        self.handlers.push(handler_fn);
        self
    }

    /// Register event handler
    /// # Notes
    /// Alias to [`Observer::register`] method
    #[inline]
    pub fn on(&mut self, handler_fn: Handler<Client>) -> &mut Self {
        self.register(handler_fn)
    }

    /// Register multiple event handlers
    /// # Notes
    /// If you want to register single handler, use [`Observer::register`] method
    pub fn registers(&mut self, handlers: impl IntoIterator<Item = Handler<Client>>) -> &mut Self {
        self.handlers.extend(handlers);
        self
    }

    /// Register filter for all handlers in the observer
    /// # Warning
    /// This filter will be applied to all handlers in the observer,
    /// if you want to apply filter to specific handler, use [`Handler::filter`] method
    pub fn filter(&mut self, val: impl Filter<Client>) -> &mut Self {
        if let Some(common) = self.common.take() {
            self.common = Some(common.filter(val));
        } else {
            let handler_fn = || async move {
                // This handler never will be called, so we can use `unreachable!` macro
                unreachable!("This handler never will be used");
                #[allow(unreachable_code)]
                Ok::<_, Infallible>(())
            };
            self.common = Some(Handler::new(handler_fn).filter(val));
        }
        self
    }
}

impl<Client> Observer<Client> {
    #[inline]
    #[must_use]
    pub fn handlers(&self) -> &[Handler<Client>] {
        &self.handlers
    }

    #[inline]
    #[must_use]
    pub fn inner_middlewares(&self) -> &[BoxedCloneInnerMiddlewareService<Client>] {
        &self.inner_middlewares.middlewares
    }

    #[inline]
    #[must_use]
    pub fn inner_middlewares_mut(&mut self) -> &mut [BoxedCloneInnerMiddlewareService<Client>] {
        &mut self.inner_middlewares.middlewares
    }

    #[inline]
    #[must_use]
    pub fn outer_middlewares(&self) -> &[BoxedCloneOuterMiddlewareService<Client>] {
        &self.outer_middlewares.middlewares
    }

    #[inline]
    #[must_use]
    pub fn outer_middlewares_mut(&mut self) -> &mut [BoxedCloneOuterMiddlewareService<Client>] {
        &mut self.outer_middlewares.middlewares
    }
}

impl<Client> Observer<Client> {
    /// Propagate event to handlers and stops propagation on first match.
    /// Handler will be called when all its filters is pass.
    /// # Errors
    /// - If any handler returns error. Probably it's error to extract args to the handler.
    #[instrument(skip_all)]
    pub async fn trigger(
        &mut self,
        request: Request<Client>,
    ) -> Result<Response<Client>, EventErrorKind>
    where
        Client: Send + Sync + Clone + 'static,
    {
        // Check observer filters
        let mut request = match self.common.as_mut() {
            Some(common) => {
                let (result, request) = common.check(request).await;
                if !result {
                    event!(Level::TRACE, "Request are not pass observer filters");

                    return Ok(Response {
                        request,
                        propagate_result: PropagateEventResult::Rejected,
                    });
                }
                request
            }
            None => request,
        };

        // Check handlers filters
        for handler in &mut self.handlers {
            let (result, new_request) = handler.check(request).await;
            request = new_request;
            if !result {
                continue;
            }

            event!(Level::TRACE, "Request are pass handler filters");

            let response = match self.inner_middlewares.middlewares.split_first_mut() {
                Some((middleware, middlewares)) => {
                    let next = wrap_to_next(
                        handler.service.clone(),
                        middlewares.to_vec().into_boxed_slice(), /* we use it instead of `into` because some versions of rustc can't infer type */
                    );
                    middleware.call((request.clone(), next)).await
                }
                None => handler
                    .call(request.clone())
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
    fn default() -> Self {
        Self::new("message")
    }
}

impl<Client> AsRef<Observer<Client>> for Observer<Client> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<Client> Clone for Observer<Client> {
    fn clone(&self) -> Self {
        Self {
            event_name: self.event_name,
            handlers: self.handlers.clone(),
            common: self.common.clone(),
            inner_middlewares: self.inner_middlewares.clone(),
            outer_middlewares: self.outer_middlewares.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Reqwest,
        errors::HandlerError,
        filters::Command,
        types::{ChatPrivate, MessageText, Update, UpdateMessage},
        Bot, Extensions,
    };

    use anyhow::anyhow;
    use std::sync::Arc;
    use tokio;

    #[allow(unreachable_code)]
    #[tokio::test]
    async fn test_observer_trigger() {
        let mut observer = Observer::default();
        // Register common filter, which handlers can't pass
        observer
            .filter(Command::one("start"))
            .register(Handler::new(|| async {
                Ok::<_, Infallible>(EventReturn::Finish)
            }))
            .register(Handler::new(|| async {
                unreachable!("It's shouldn't trigger because the first handler handles the event");

                Ok::<_, Infallible>(EventReturn::Finish)
            }));

        let mut request = Request::<Reqwest> {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(0), ""),
            ))),
            bot: Bot::default(),
            context: crate::Context::default(),
            extensions: Extensions::default(),
        };
        let response = observer.trigger(request.clone()).await.unwrap();

        // Filter not pass, so handler should be rejected
        match response.propagate_result {
            PropagateEventResult::Rejected => {}
            _ => panic!("Unexpected result"),
        }

        request.update = Arc::new(Update::Message(UpdateMessage::new(
            0,
            MessageText::new(0, 0, ChatPrivate::new(0), "/start"),
        )));

        let response = observer.trigger(request).await.unwrap();

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
        observer
            .register(Handler::new(|| async {
                Err::<(), _>(HandlerError::new(anyhow!("test")))
            }))
            .register(Handler::new(|| async {
                unreachable!("It's shouldn't trigger because the first handler handles the event");

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
        let response = observer.trigger(request).await.unwrap();

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
        observer
            .register(Handler::new(|| async {
                Ok::<_, Infallible>(EventReturn::Skip)
            }))
            .register(Handler::new(|| async {
                Ok::<_, Infallible>(EventReturn::Finish)
            }));

        let request = Request::<Reqwest> {
            update: Arc::new(Update::Message(UpdateMessage::new(
                0,
                MessageText::new(0, 0, ChatPrivate::new(0), "/start"),
            ))),
            bot: Bot::default(),
            context: crate::Context::default(),
            extensions: Extensions::default(),
        };
        let response = observer.trigger(request.clone()).await.unwrap();

        // First handler returns `EventReturn::Skip`, so second handler should be called
        match response.propagate_result {
            PropagateEventResult::Handled(response) => match response.handler_result {
                Ok(EventReturn::Finish) => {}
                _ => panic!("Unexpected result"),
            },
            _ => panic!("Unexpected result"),
        }

        let mut observer = Observer::default();
        observer
            .register(Handler::new(|| async {
                Ok::<_, Infallible>(EventReturn::Skip)
            }))
            .register(Handler::new(|| async {
                Ok::<_, Infallible>(EventReturn::Cancel)
            }));

        let response = observer.trigger(request).await.unwrap();

        // First handler returns `EventReturn::Skip`, so second handler should be called and it returns `EventReturn::Cancel`,
        // so response should be `PropagateEventResult::Rejected`
        match response.propagate_result {
            PropagateEventResult::Rejected => {}
            _ => panic!("Unexpected result"),
        }
    }
}
