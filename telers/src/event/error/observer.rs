use super::{event::ErrorEvent, handler::Handler};
use crate::{
    errors::HandlerError,
    event::{service::Service, EventReturn},
};

use std::fmt::{self, Debug, Formatter};
use tracing::{event, instrument, Level};

/// Whether the error was handled by one of the registered error handlers.
pub enum PropagateErrorResult<Client> {
    /// Error was handled by a handler
    Handled,
    /// No handler processed the error; the event is returned back for further propagation
    Unhandled(ErrorEvent<Client>),
}

/// Error events observer.
///
/// Unlike telegram observers, error observers have no filters or middlewares —
/// error handlers are meant to be a simple, fast last line of defense.
///
/// Handlers are called in order of registration;
/// a handler returning [`EventReturn::Skip`] passes the error to the next handler,
/// any other variant stops propagation and marks the error as handled.
pub struct Observer<Client> {
    pub(crate) event_name: &'static str,
    pub(crate) handlers: Vec<Handler<Client>>,
}

impl<Client> Clone for Observer<Client> {
    fn clone(&self) -> Self {
        Self {
            event_name: self.event_name,
            handlers: self.handlers.clone(),
        }
    }
}

impl<Client> Observer<Client>
where
    Client: Send + Sync + 'static,
{
    #[inline]
    #[must_use]
    pub const fn new(event_name: &'static str) -> Self {
        Self {
            event_name,
            handlers: vec![],
        }
    }

    /// Register error handler
    #[inline]
    #[must_use]
    pub fn register(mut self, handler: Handler<Client>) -> Self {
        self.handlers.push(handler);
        self
    }

    /// Register error handler
    /// # Notes
    /// Alias to [`Observer::register`] method
    #[inline]
    #[must_use]
    pub fn on(self, handler: Handler<Client>) -> Self {
        self.register(handler)
    }

    /// Register multiple error handlers
    /// # Notes
    /// If you want to register single handler, use [`Observer::register`] method
    #[must_use]
    pub fn registers(mut self, handlers: impl IntoIterator<Item = Handler<Client>>) -> Self {
        self.handlers.extend(handlers);
        self
    }

    #[inline]
    #[must_use]
    pub fn handlers_len(&self) -> usize {
        self.handlers.len()
    }

    /// Propagate the error to handlers in order of registration.
    /// Stops on the first handler that doesn't return [`EventReturn::Skip`].
    /// # Errors
    /// If any handler returns an error
    #[instrument(skip_all)]
    pub async fn trigger(
        &mut self,
        event: ErrorEvent<Client>,
    ) -> Result<PropagateErrorResult<Client>, HandlerError>
    where
        Client: Clone + Send + Sync + 'static,
    {
        for handler in &mut self.handlers {
            match handler.call(event.clone()).await? {
                EventReturn::Skip => {
                    event!(Level::TRACE, "Error handler returns skip");
                }
                EventReturn::Finish | EventReturn::Cancel => {
                    event!(Level::TRACE, "Error handler handles the error");
                    return Ok(PropagateErrorResult::Handled);
                }
            }
        }

        event!(
            Level::TRACE,
            "Error not handled by any handler in this observer"
        );
        Ok(PropagateErrorResult::Unhandled(event))
    }
}

impl<Client> Debug for Observer<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Observer")
            .field("event_name", &self.event_name)
            .field("handlers", &self.handlers.len())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::Reqwest, errors::EventErrorKind, event::EventReturn, Bot, Request};

    use anyhow::anyhow;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    use tokio;

    fn test_event() -> ErrorEvent<Reqwest> {
        ErrorEvent::new(
            Request::<Reqwest> {
                update: Arc::new(crate::types::Update::Message(
                    crate::types::UpdateMessage::new(
                        0,
                        crate::types::MessageText::new(0, 0, crate::types::ChatPrivate::new(0), ""),
                    ),
                )),
                bot: Bot::default(),
                context: crate::Context::default(),
                extensions: crate::Extensions::default(),
            },
            Arc::new(EventErrorKind::Handler(crate::errors::HandlerError::new(
                anyhow!("test"),
            ))),
        )
    }

    #[tokio::test]
    async fn test_trigger_order_and_handled() {
        let calls = Arc::new(AtomicUsize::new(0));
        let first_calls = Arc::clone(&calls);
        let second_calls = Arc::clone(&calls);

        let mut observer = Observer::new("error")
            .register(Handler::new(move |_event: ErrorEvent<Reqwest>| {
                let calls = Arc::clone(&first_calls);
                async move {
                    calls.fetch_add(1, Ordering::SeqCst);
                    Ok(EventReturn::Finish)
                }
            }))
            .register(Handler::new(move |_event: ErrorEvent<Reqwest>| {
                let calls = Arc::clone(&second_calls);
                async move {
                    calls.fetch_add(1, Ordering::SeqCst);
                    Ok(EventReturn::Finish)
                }
            }));

        let result = observer.trigger(test_event()).await.unwrap();

        assert!(matches!(result, PropagateErrorResult::Handled));
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_trigger_skip_goes_to_next_handler() {
        let calls = Arc::new(AtomicUsize::new(0));
        let first_calls = Arc::clone(&calls);
        let second_calls = Arc::clone(&calls);

        let mut observer = Observer::new("error")
            .register(Handler::new(move |_event: ErrorEvent<Reqwest>| {
                let calls = Arc::clone(&first_calls);
                async move {
                    calls.fetch_add(1, Ordering::SeqCst);
                    Ok(EventReturn::Skip)
                }
            }))
            .register(Handler::new(move |_event: ErrorEvent<Reqwest>| {
                let calls = Arc::clone(&second_calls);
                async move {
                    calls.fetch_add(1, Ordering::SeqCst);
                    Ok(EventReturn::Finish)
                }
            }));

        let result = observer.trigger(test_event()).await.unwrap();

        assert!(matches!(result, PropagateErrorResult::Handled));
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_trigger_unhandled() {
        let mut observer = Observer::<Reqwest>::new("error")
            .register(Handler::new(|_event| async { Ok(EventReturn::Skip) }));

        let event = test_event();
        let result = observer.trigger(event.clone()).await.unwrap();

        match result {
            PropagateErrorResult::Unhandled(unhandled) => {
                assert!(Arc::ptr_eq(&unhandled.error, &event.error));
            }
            _ => panic!("Unexpected result"),
        }
    }

    #[tokio::test]
    async fn test_trigger_no_handlers() {
        let mut observer = Observer::<Reqwest>::new("error");

        let event = test_event();
        let result = observer.trigger(event.clone()).await.unwrap();

        match result {
            PropagateErrorResult::Unhandled(unhandled) => {
                assert!(Arc::ptr_eq(&unhandled.error, &event.error));
            }
            _ => panic!("Unexpected result"),
        }
    }

    #[tokio::test]
    async fn test_trigger_handler_error() {
        let mut observer =
            Observer::<Reqwest>::new("error").register(Handler::new(|_event| async {
                Err(HandlerError::new(anyhow!("error handler failed")))
            }));

        assert!(observer.trigger(test_event()).await.is_err());
    }
}
