use super::{
    service::{Service, ServiceFactory},
    EventHandler, EventHandlerObject, EventHandlerObjectService,
};

use crate::error::app;

use futures::future::join_all;
use futures_core::future::LocalBoxFuture;
use std::rc::Rc;

/// Simple events observer
/// Is used for managing events isn't related with Telegram (For example startup/shutdown processes)
#[derive(Default)]
pub struct EventObserver {
    /// Handlers of the observer
    handlers: Vec<EventHandlerObject>,
}

impl EventObserver {
    /// Creates a new event observer
    #[must_use]
    pub fn new() -> Self {
        Self { handlers: vec![] }
    }

    /// Get handlers of the observer
    #[must_use]
    fn handlers(&self) -> &[EventHandlerObject] {
        &self.handlers
    }

    /// Add a handler to the observer
    /// # Arguments
    /// * `handler` - Handler for the observer
    fn register<H, Args>(&mut self, handler: H, args: Args)
    where
        H: EventHandler<Args> + 'static,
        Args: Clone + 'static,
    {
        self.handlers.push(EventHandlerObject::new(handler, args));
    }
}

impl ServiceFactory<()> for EventObserver {
    type Response = ();
    type Error = app::Error;
    type Config = ();
    type Service = ObserverService;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    fn new_service(&self, _: Self::Config) -> Self::Future {
        let futs = self
            .handlers
            .iter()
            .map(|handler| handler.new_service(()))
            .collect::<Vec<_>>();

        Box::pin(async move {
            let handlers = join_all(futs).await.into_iter().collect::<Result<_, _>>()?;

            Ok(ObserverService {
                handlers: Rc::new(handlers),
            })
        })
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct ObserverService {
    /// Handler services of the observer
    handlers: Rc<Vec<EventHandlerObjectService>>,
}

impl ObserverService {
    async fn trigger(&self, req: ()) -> Result<(), app::Error> {
        ObserverService::trigger_without_self(Rc::clone(&self.handlers), req).await
    }

    /// We need this method to possible boxed without [`ObserverService`] lifetime
    #[allow(clippy::similar_names)]
    async fn trigger_without_self(
        handlers: Rc<Vec<EventHandlerObjectService>>,
        _: (),
    ) -> Result<(), app::Error> {
        for handler in handlers.iter() {
            match handler.call(()).await {
                Ok(_) => {}
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(())
    }
}

impl Service<()> for ObserverService {
    type Response = ();
    type Error = app::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: ()) -> Self::Future {
        Box::pin(ObserverService::trigger_without_self(
            Rc::clone(&self.handlers),
            req,
        ))
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

        let mut observer = EventObserver::new();

        observer.register(on_startup, ("Hello, world!",));
        observer.register(on_shutdown, ("Goodbye, world!",));

        let observer_service = r#await!(observer.new_service(())).unwrap();

        r#await!(observer_service.call(())).unwrap();
    }
}
