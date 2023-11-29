use crate::event::{
    service::{Service as _, ServiceFactory as _, ServiceProvider, ToServiceProvider},
    simple::handler::{Handler, HandlerObject, HandlerObjectService, Result as HandlerResult},
};

use std::fmt::{self, Debug, Formatter};
use tracing::instrument;

/// Simple events observer
/// Is used for managing events isn't related with Telegram (For example startup/shutdown events)
pub struct Observer {
    pub event_name: &'static str,

    handlers: Vec<HandlerObject>,
}

impl Observer {
    #[must_use]
    pub fn new(event_name: &'static str) -> Self {
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

impl Default for Observer {
    #[must_use]
    fn default() -> Self {
        Self::new("default")
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
        let event_name = self.event_name;
        let handlers = self
            .handlers
            .iter()
            .map(|handler| handler.new_service(config))
            .collect::<Result<_, _>>()?;

        Ok(Service {
            event_name,
            handlers,
        })
    }
}

pub struct Service {
    event_name: &'static str,
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

        let mut observer = Observer::new("test");
        observer.register(on_startup, ("Hello, world!",));
        observer.register(on_shutdown, ("Goodbye, world!",));

        let observer_service = observer.to_service_provider_default().unwrap();

        observer_service.trigger(()).await.unwrap();
    }

    #[tokio::test]
    async fn test_observer_trigger_error() {
        async fn on_startup(message: &str) -> HandlerResult {
            assert_eq!(message, "Hello, world!");

            Ok(())
        }

        async fn on_shutdown(message: &str) -> HandlerResult {
            assert_eq!(message, "Goodbye, world!");

            Err(HandlerError::new(anyhow!("test")))
        }

        let mut observer = Observer::new("test");
        observer.register(on_startup, ("Hello, world!",));
        observer.register(on_shutdown, ("Goodbye, world!",));

        let observer_service = observer.to_service_provider_default().unwrap();

        observer_service.trigger(()).await.unwrap_err();
    }
}
