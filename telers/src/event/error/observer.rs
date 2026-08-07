use super::{
    event::ErrorEvent,
    handler::handler,
};
use crate::{errors::HandlerError, event::EventReturn};
use std::fmt::{self, Debug, Formatter};
use tracing::{event, instrument, Level};

/// Whether the error was handled by one of the registered error handlers.
pub enum PropaGateErrorResult<Client> {
    Handled,
    Unhandled(ErrorEvent<Client>),
}

/// Observer for errors that occur while processing an update.
/// Handlers are called in order of registration; a handler returning
/// [`EventReturn::Skip`] passes the error to the next handler.
/// Unlike telegram handlers, error handlers have no filters or middlewares —
/// they're meant to be a simple, fast last line of defense.
#[derive(Clone)]
pub struct Observer<Client> {
    pub(crate) event_name: &'static str,
    pub(crate) handlers: Vec<Handler<Client>>,
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

    #[inline]
    #[must_use]
    pub fn on(self, handler: Handler<Client>) -> Self {
        self.register(handler)
    }


    /// Register error handler
    /// # Notes
    /// Alias to [`Observer::register`] method
    #[must_use]
    pub fn registers(mut self, handlers: impl IntoIterator<Item = Handler<Client>>) -> Self {
        self.handlers.extend(handlers);
        self
    }
}

impl<Client> Observer<Client> {
    #[inline]
    #[must_use]
    pub fn handlers_len(&self) -> usize {
        self.handlers.len()
    }
}

impl<Client> Observer<Client> 
where
Client: Clone + Send + Sync + 'static,
{

    /// Propagate the error to handlers in order of registration.
    /// Stops on the first handler that doesn't return [`EventReturn::Skip`].
    /// # Errors
    /// If any handler returns an error
    #[instrument(skip_all)]
    pub async fn trigger(
        &mut self,
        event: ErrorEvent<Client>,
    ) -> Result<PropaGateErrorResult<Client>, HandlerError> {
        
        for handler in &mut self.handlers {
            match handler.call(event.clone()).await? {
                EventReturn::Skip => {
                    event!(Level::TRACE, "Error handler return skip.");
                    continue;
                }
                EventReturn::Finish | EventReturn::Cancel => {
                    event!(Level::TRACE, "Error handler returns Finish/Cancel");
                    return Ok(PropaGateErrorResult::Handled);
                }
            }
        }

        event!(Level::TRACE, "Error not handled by any handler in this observer");
        Ok(PropaGateErrorResult::Unhandled(event))
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