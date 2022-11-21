use crate::{
    dispatcher::event::{
        service::{Service, ServiceFactory},
        simple::{Handler, HandlerObject, HandlerObjectService},
    },
    error::app,
};

use futures_core::future::LocalBoxFuture;
use std::{
    fmt::{self, Debug, Formatter},
    rc::Rc,
};

/// Simple events observer
/// Is used for managing events isn't related with Telegram (For example startup/shutdown processes)
#[derive(Default)]
pub struct Observer {
    /// Event observer name
    event_name: &'static str,
    /// Handlers of the observer
    handlers: Vec<HandlerObject>,
}

impl Observer {
    /// Creates a new event observer
    /// # Arguments
    /// * `event_name` - Event observer name, can be used for logging
    #[must_use]
    pub fn new(event_name: &'static str) -> Self {
        Self {
            event_name,
            handlers: vec![],
        }
    }

    /// Get event observer name
    #[must_use]
    pub fn event_name(&self) -> &str {
        self.event_name
    }

    /// Get handlers of the observer
    #[must_use]
    pub fn handlers(&self) -> &[HandlerObject] {
        &self.handlers
    }

    /// Register event handler
    /// # Arguments
    /// * `handler` - Handler for the observer
    pub fn register<H, Args>(&mut self, handler: H, args: Args)
    where
        H: Handler<Args> + 'static,
        Args: Clone + 'static,
    {
        self.handlers.push(HandlerObject::new(handler, args));
    }
}

impl AsRef<Observer> for Observer {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl ServiceFactory<()> for Observer {
    type Response = ();
    type Error = app::Error;
    type Config = ();
    type Service = ObserverService;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    /// Create [`ObserverService`] from [`Observer`]
    fn new_service(&self, _: Self::Config) -> Self::Future {
        let event_name = self.event_name;
        let futs = self
            .handlers
            .iter()
            .map(|handler| handler.new_service(()))
            .collect::<Vec<_>>();

        Box::pin(async move {
            let mut handlers = vec![];
            for fut in futs {
                handlers.push(fut.await?);
            }

            Ok(ObserverService {
                event_name,
                handlers: Rc::new(handlers),
            })
        })
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct ObserverService {
    /// Event observer name
    event_name: &'static str,
    /// Handler services of the observer
    handlers: Rc<Vec<HandlerObjectService>>,
}

impl ObserverService {
    /// Propagate event to handlers. All handlers will be called.
    /// # Errors
    /// If any handler returns error
    pub async fn trigger(&self, _: ()) -> Result<(), app::Error> {
        for handler in self.handlers.iter() {
            handler.call(()).await?;
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
    type Error = app::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, _: ()) -> Self::Future {
        log::error!("{:?}: Should not be called", self);

        unimplemented!(
            "ObserverService is not intended to be called directly. \
            Use ObserverService::trigger instead"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! r#await {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_observer_trigger() {
        async fn on_startup(message: &str) -> Result<(), app::Error> {
            assert_eq!(message, "Hello, world!");
            Ok(())
        }

        async fn on_shutdown(message: &str) -> Result<(), app::Error> {
            assert_eq!(message, "Goodbye, world!");
            Ok(())
        }

        let mut observer = Observer::new("test");
        observer.register(on_startup, ("Hello, world!",));
        observer.register(on_shutdown, ("Goodbye, world!",));

        let observer_service = r#await!(observer.new_service(())).unwrap();

        r#await!(observer_service.trigger(())).unwrap();
    }
}
