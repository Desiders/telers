use crate::{
    dispatcher::event::{
        service::{Service, ServiceFactory},
        simple::{Handler, HandlerObject, HandlerObjectService},
    },
    error::app,
};

use futures_core::future::LocalBoxFuture;
use std::rc::Rc;

/// Simple events observer
/// Is used for managing events isn't related with Telegram (For example startup/shutdown processes)
#[derive(Default)]
pub struct Observer {
    /// Handlers of the observer
    handlers: Vec<HandlerObject>,
}

impl Observer {
    /// Creates a new event observer
    #[must_use]
    pub fn new() -> Self {
        Self { handlers: vec![] }
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
                handlers: Rc::new(handlers),
            })
        })
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct ObserverService {
    /// Handler services of the observer
    handlers: Rc<Vec<HandlerObjectService>>,
}

impl ObserverService {
    /// Propagate event to handlers.
    /// All handlers will be called.
    /// # Errors
    /// If handler service returns error.
    pub async fn trigger(&self, req: ()) -> Result<(), app::Error> {
        Self::trigger_without_self(Rc::clone(&self.handlers), req).await
    }

    /// We need this method to possible call without [`ObserverService`] lifetime
    #[allow(clippy::similar_names)]
    async fn trigger_without_self(
        handlers: Rc<Vec<HandlerObjectService>>,
        _: (),
    ) -> Result<(), app::Error> {
        for handler in handlers.iter() {
            handler.call(()).await?;
        }
        Ok(())
    }
}

impl Service<()> for ObserverService {
    type Response = ();
    type Error = app::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: ()) -> Self::Future {
        Box::pin(Self::trigger_without_self(Rc::clone(&self.handlers), req))
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

        let mut observer = Observer::new();
        observer.register(on_startup, ("Hello, world!",));
        observer.register(on_shutdown, ("Goodbye, world!",));

        let observer_service = r#await!(observer.new_service(())).unwrap();

        r#await!(observer_service.call(())).unwrap();
    }
}
