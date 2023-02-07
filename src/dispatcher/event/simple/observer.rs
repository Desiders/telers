use crate::dispatcher::event::{
    service::{BoxFuture, Service, ServiceFactory},
    simple::handler::{Handler, HandlerObject, HandlerObjectService, Result as HandlerResult},
};

use std::{
    fmt::{self, Debug, Formatter},
    sync::Arc,
};

/// Simple events observer
/// Is used for managing events isn't related with Telegram (For example startup/shutdown events)
pub struct Observer {
    event_name: &'static str,
    handlers: Vec<HandlerObject>,
}

impl Observer {
    /// # Arguments
    /// * `event_name` - Event observer name, can be used for logging
    #[must_use]
    pub fn new(event_name: &'static str) -> Self {
        Self {
            event_name,
            handlers: vec![],
        }
    }

    #[must_use]
    pub fn event_name(&self) -> &str {
        self.event_name
    }

    #[must_use]
    pub fn handlers(&self) -> &[HandlerObject] {
        &self.handlers
    }

    /// Register event handler
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `args` - Arguments, that will be passed to the handler
    pub fn register<H, Args>(&mut self, handler: H, args: Args)
    where
        H: Handler<Args> + Clone + Send + Sync + 'static,
        H::Future: Send,
        H::Output: Into<HandlerResult>,
        Args: Clone + Send + Sync + 'static,
    {
        self.handlers.push(HandlerObject::new(handler, args));
    }

    // Alias to [`Observer::register`] method
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
    fn default() -> Self {
        Self::new("default")
    }
}

impl AsRef<Observer> for Observer {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl ServiceFactory<()> for Observer {
    type Response = ();
    type Error = ();
    type Config = ();
    type Service = ObserverService;
    type InitError = ();

    fn new_service(&self, config: Self::Config) -> Result<Self::Service, Self::InitError> {
        let event_name = self.event_name;
        let handlers = self
            .handlers
            .iter()
            .map(|handler| handler.new_service(config))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ObserverService {
            event_name,
            handlers: Arc::new(handlers),
        })
    }
}

#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct ObserverService {
    /// Event observer name, can be used for logging
    event_name: &'static str,
    /// Handler services of the observer
    handlers: Arc<Vec<HandlerObjectService>>,
}

impl ObserverService {
    /// Propagate event to handlers. \
    /// If any handler returns error, then propagation will be stopped and error will be returned.
    /// # Errors
    /// If any handler returns error
    pub async fn trigger(&self, request: ()) -> HandlerResult {
        for handler in self.handlers.iter() {
            handler.call(request).await?;
        }

        Ok(())
    }
}

impl Debug for ObserverService {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ObserverService")
            .field("event_name", &self.event_name)
            .finish()
    }
}

impl Service<()> for ObserverService {
    type Response = ();
    type Error = ();
    type Future = BoxFuture<Result<Self::Response, Self::Error>>;

    fn call(&self, _: ()) -> Self::Future {
        log::error!("{self:?}: Should not be called");

        unimplemented!(
            "ObserverService is not intended to be called directly. \
            Use ObserverService::trigger instead"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let observer_service = observer.new_service(()).unwrap();

        observer_service.trigger(()).await.unwrap();
    }
}
