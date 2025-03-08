use crate::{
    enums::SimpleObserverName,
    event::{
        service::Service,
        simple::handler::{Handler, HandlerComposite, HandlerResult},
    },
};

use std::fmt::{self, Debug, Formatter};
use tracing::instrument;

/// Simple events observer
/// Is used for managing events isn't related with Telegram (For example startup/shutdown events)
#[derive(Clone)]
pub struct Observer {
    pub event_name: SimpleObserverName,
    handlers: Vec<HandlerComposite>,
}

impl Observer {
    #[inline]
    #[must_use]
    pub const fn new(event_name: SimpleObserverName) -> Self {
        Self {
            event_name,
            handlers: vec![],
        }
    }

    #[inline]
    #[must_use]
    pub fn handlers(&self) -> &[HandlerComposite] {
        &self.handlers
    }

    /// Register event handler
    pub fn register<H, Args>(&mut self, handler: H, args: Args)
    where
        H: Handler<Args>,
        Args: Clone + Send + Sync + 'static,
    {
        self.handlers.push(HandlerComposite::new(handler, args));
    }

    /// Register service as event handler
    pub fn register_service<S, Args>(&mut self, service: S, args: Args)
    where
        S: Service<Args, Response = ()> + Clone + Send + Sync + 'static,
        S::Error: Into<anyhow::Error> + Send + Sync + 'static,
        S::Future: Send,
        Args: Clone + Send + Sync + 'static,
    {
        self.handlers
            .push(HandlerComposite::new_service(service, args));
    }

    /// Alias to [`Observer::register`] method
    #[inline]
    pub fn on<H, Args>(&mut self, handler: H, args: Args)
    where
        H: Handler<Args>,
        Args: Clone + Send + Sync + 'static,
    {
        self.register(handler, args);
    }

    /// Alias to [`Observer::register_service`] method
    #[inline]
    pub fn on_service<S, Args>(&mut self, service: S, args: Args)
    where
        S: Service<Args, Response = ()> + Clone + Send + Sync + 'static,
        S::Error: Into<anyhow::Error> + Send + Sync + 'static,
        S::Future: Send,
        Args: Clone + Send + Sync + 'static,
    {
        self.register_service(service, args);
    }
}

impl Observer {
    #[allow(clippy::let_unit_value)]
    #[instrument(skip(self, request))]
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

        let mut startup_observer = Observer::new(SimpleObserverName::Startup);
        startup_observer.register(on_startup, ("Hello, world!",));

        let mut shutdown_observer = Observer::new(SimpleObserverName::Shutdown);
        shutdown_observer.register(on_shutdown, ("Goodbye, world!",));

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

        let mut startup_observer = Observer::new(SimpleObserverName::Startup);
        startup_observer.register(on_startup, ("Hello, world!",));

        let mut shutdown_observer = Observer::new(SimpleObserverName::Shutdown);
        shutdown_observer.register(on_shutdown, ("Goodbye, world!",));

        assert!(
            startup_observer.trigger(()).await.is_err()
                && shutdown_observer.trigger(()).await.is_err()
        );
    }
}
