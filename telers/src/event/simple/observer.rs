use crate::event::{
    service::Service,
    simple::handler::{Handler, HandlerResult},
};

use std::fmt::{self, Debug, Formatter};

/// Simple events observer
/// Is used for managing events isn't related with Telegram (For example startup/shutdown events)
#[derive(Clone)]
pub struct Observer {
    pub event_name: &'static str,
    handlers: Vec<Handler>,
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
    pub fn register<H>(&mut self, handler: H) -> &mut Self
    where
        H: Into<Handler>,
    {
        self.handlers.push(handler.into());
        self
    }

    /// Register event handler
    /// # Notes
    /// Alias to [`Observer::register`] method
    #[inline]
    pub fn on<H>(&mut self, handler: H) -> &mut Self
    where
        H: Into<Handler>,
    {
        self.register(handler)
    }
}

impl Observer {
    #[inline]
    #[must_use]
    pub fn handlers(&self) -> &[Handler] {
        &self.handlers
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
            .finish_non_exhaustive()
    }
}

impl AsRef<Observer> for Observer {
    fn as_ref(&self) -> &Self {
        self
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
        startup_observer.register(Handler::new(on_startup, ("Hello, world!",)));

        let mut shutdown_observer = Observer::new("shutdown");
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
        startup_observer.register(Handler::new(on_startup, ("Hello, world!",)));

        let mut shutdown_observer = Observer::new("shutdown");
        shutdown_observer.register(Handler::new(on_shutdown, ("Goodbye, world!",)));

        assert!(
            startup_observer.trigger(()).await.is_err()
                && shutdown_observer.trigger(()).await.is_err()
        );
    }
}
