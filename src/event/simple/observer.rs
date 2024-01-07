use crate::{
    enums::SimpleObserverName,
    event::{
        service::{Service as _, ServiceFactory as _, ServiceProvider, ToServiceProvider},
        simple::handler::{Handler, HandlerObject, HandlerObjectService, Result as HandlerResult},
    },
};

use std::fmt::{self, Debug, Formatter};
use tracing::instrument;

/// Simple events observer
/// Is used for managing events isn't related with Telegram (For example startup/shutdown events)
pub struct Observer {
    pub event_name: SimpleObserverName,

    handlers: Vec<HandlerObject>,
}

impl Observer {
    #[must_use]
    pub fn new(event_name: SimpleObserverName) -> Self {
        Self {
            event_name,
            handlers: vec![],
        }
    }

    #[must_use]
    pub fn handlers(&self) -> &[HandlerObject] {
        &self.handlers
    }

    /// Register event handler
    pub fn register<H, Args>(&mut self, handler: H, args: Args)
    where
        H: Handler<Args> + Clone + Send + Sync + 'static,
        H::Future: Send,
        H::Output: Into<HandlerResult>,
        Args: Clone + Send + Sync + 'static,
    {
        self.handlers.push(HandlerObject::new(handler, args));
    }

    /// Alias to [`Observer::register`] method
    pub fn on<H, Args>(&mut self, handler: H, args: Args)
    where
        H: Handler<Args> + Clone + Send + Sync + 'static,
        H::Future: Send,
        H::Output: Into<HandlerResult>,
        Args: Clone + Send + Sync + 'static,
    {
        self.register(handler, args);
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

impl ToServiceProvider for Observer {
    type Config = ();
    type ServiceProvider = Service;
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
        })
    }
}

pub struct Service {
    event_name: SimpleObserverName,
    handlers: Box<[HandlerObjectService]>,
}

impl Debug for Service {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Service")
            .field("event_name", &self.event_name)
            .finish_non_exhaustive()
    }
}

impl ServiceProvider for Service {}

impl Service {
    /// Propagate event to handlers
    ///
    /// If any handler returns error, then propagation will be stopped and error will be returned.
    /// # Errors
    /// If any handler returns error
    #[allow(clippy::let_unit_value)]
    #[instrument(skip(self, request))]
    pub async fn trigger(&self, request: ()) -> HandlerResult {
        for handler in &*self.handlers {
            handler.call(request).await?;
        }

        Ok(())
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

        let startup_observer = startup_observer.to_service_provider_default().unwrap();
        let shutdown_observer = shutdown_observer.to_service_provider_default().unwrap();

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

        let startup_observer = startup_observer.to_service_provider_default().unwrap();
        let shutdown_observer = shutdown_observer.to_service_provider_default().unwrap();

        assert!(
            startup_observer.trigger(()).await.is_err()
                && shutdown_observer.trigger(()).await.is_err()
        );
    }
}
