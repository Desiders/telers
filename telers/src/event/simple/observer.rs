use crate::event::{
    service::Service,
    simple::handler::{Handler, HandlerResult},
};

use std::fmt::{self, Debug, Formatter};

/// Simple events observer
/// Is used for managing events isn't related with Telegram (For example startup/shutdown events)
#[derive(Clone)]
pub struct Observer {
    pub(crate) event_name: &'static str,
    pub(crate) handlers: Vec<Handler>,
}

impl Observer {
    #[inline]
    #[must_use]
    pub const fn new(event_name: &'static str) -> Self {
        Self {
            event_name,
            handlers: vec![],
        }
    }

    /// Register event handler
    #[inline]
    #[must_use]
    pub fn register(mut self, handler: Handler) -> Self {
        self.handlers.push(handler);
        self
    }

    /// Register event handler
    /// # Notes
    /// Alias to [`Observer::register`] method
    #[inline]
    #[must_use]
    pub fn on(self, handler: Handler) -> Self {
        self.register(handler)
    }

    /// Register multiple event handlers
    /// # Notes
    /// If you want to register single handler, use [`Observer::register`] method
    #[must_use]
    pub fn registers(mut self, handlers: impl IntoIterator<Item = Handler>) -> Self {
        self.handlers.extend(handlers);
        self
    }
}

impl Observer {
    #[inline]
    #[must_use]
    pub fn handlers_len(&self) -> usize {
        self.handlers.len()
    }
}

impl Observer {
    #[allow(clippy::missing_errors_doc)]
    pub async fn trigger(&mut self, request: ()) -> HandlerResult {
        for handler in &mut self.handlers {
            handler.call(request).await?;
        }
        Ok(())
    }
}

impl Debug for Observer {
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
    use crate::errors::HandlerError;

    use anyhow::anyhow;
    use tokio;

    #[tokio::test]
    async fn test_observer_trigger() {
        async fn on_startup(message: &str) -> HandlerResult {
            assert_eq!(message, "Hello, world!");

            Ok(())
        }

        async fn on_shutdown(message: &str) -> HandlerResult {
            assert_eq!(message, "Goodbye, world!");

            Ok(())
        }

        let mut startup_observer = Observer::new("startup");
        startup_observer = startup_observer.register(Handler::new(on_startup, ("Hello, world!",)));

        let mut shutdown_observer = Observer::new("shutdown");
        shutdown_observer =
            shutdown_observer.register(Handler::new(on_shutdown, ("Goodbye, world!",)));

        startup_observer.trigger(()).await.unwrap();
        shutdown_observer.trigger(()).await.unwrap();
    }

    #[tokio::test]
    async fn test_observer_trigger_error() {
        async fn on_startup(message: &str) -> HandlerResult {
            assert_eq!(message, "Hello, world!");

            Err(HandlerError::new(anyhow!("test")))
        }

        async fn on_shutdown(message: &str) -> HandlerResult {
            assert_eq!(message, "Goodbye, world!");

            Err(HandlerError::new(anyhow!("test")))
        }

        let mut startup_observer = Observer::new("startup");
        startup_observer = startup_observer.register(Handler::new(on_startup, ("Hello, world!",)));

        let mut shutdown_observer = Observer::new("shutdown");
        shutdown_observer =
            shutdown_observer.register(Handler::new(on_shutdown, ("Goodbye, world!",)));

        assert!(
            startup_observer.trigger(()).await.is_err()
                && shutdown_observer.trigger(()).await.is_err()
        );
    }
}
