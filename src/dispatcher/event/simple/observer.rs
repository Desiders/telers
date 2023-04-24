use crate::dispatcher::event::{
    service::{Service as _, ServiceFactory as _, ServiceProvider, ToServiceProvider},
    simple::handler::{Handler, HandlerObject, HandlerObjectService, Result as HandlerResult},
};

use std::fmt::{self, Debug, Formatter};

/// Simple events observer
/// Is used for managing events isn't related with Telegram (For example startup/shutdown events)
pub struct Observer {
    /// Can be used for logging and debugging
    pub event_name: &'static str,
    pub handlers: Vec<HandlerObject>,
}

impl Observer {
    #[must_use]
    pub fn new(event_name: &'static str) -> Self {
        Self {
            event_name,
            handlers: vec![],
        }
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
            .collect::<Result<_, _>>()?;

        Ok(ObserverInner {
            event_name,
            handlers,
        })
    }
}

pub struct ObserverInner {
    event_name: &'static str,
    handlers: Vec<HandlerObjectService>,
}

impl Debug for ObserverInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ObserverInner")
            .field("event_name", &self.event_name)
            .finish()
    }
}

impl ServiceProvider for ObserverInner {}

impl ObserverInner {
    /// Propagate event to handlers
    ///
    /// If any handler returns error, then propagation will be stopped and error will be returned.
    /// # Errors
    /// If any handler returns error
    pub async fn trigger(&self, request: ()) -> HandlerResult {
        for handler in &self.handlers {
            handler.call(request).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::EventError;

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

            Err(EventError::new(anyhow!("test")))
        }

        let mut observer = Observer::new("test");
        observer.register(on_startup, ("Hello, world!",));
        observer.register(on_shutdown, ("Goodbye, world!",));

        let observer_service = observer.to_service_provider_default().unwrap();

        observer_service.trigger(()).await.unwrap_err();
    }
}
