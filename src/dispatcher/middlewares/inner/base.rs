use crate::{
    dispatcher::event::{
        service::BoxFuture,
        telegram::handler::{BoxedHandlerService, Request, Response},
    },
    error::app,
};

pub type MiddlewareType = Box<dyn Middleware + Send + Sync>;
pub type MiddlewaresType = Vec<Arc<MiddlewareType>>;
pub type NextMiddlewaresIterType = Box<dyn Iterator<Item = Arc<MiddlewareType>> + Send + Sync>;

use std::{future::Future, sync::Arc};

/// A base trait for middlewares
pub trait Middleware: Send + Sync {
    /// Execute middleware
    /// # Arguments
    /// * `handler` - Handler service
    /// * `req` - Data for handler service
    /// * `middlewares` - Middlewares for handler service
    /// # Returns
    /// [`Response`] from handler service or [`app::ErrorKind`]
    /// # Errors
    /// If any inner middleware returns error
    /// If handler returns error. Probably it's error to extract args to the handler
    #[must_use]
    fn call(
        &self,
        handler: Arc<BoxedHandlerService>,
        req: Request,
        middlewares: NextMiddlewaresIterType,
    ) -> BoxFuture<Result<Response, app::ErrorKind>>;

    /// Call next middleware or handler service if all middlewares has passed
    /// # Arguments
    /// * `handler` - Handler service
    /// * `req` - Data for handler service
    /// * `middlewares` - Middlewares for handler service
    /// # Returns
    /// [`Response`] from handler service or [`app::ErrorKind`]
    /// # Errors
    /// If any inner middleware returns error
    /// If handler returns error. Probably it's error to extract args to the handler
    #[must_use]
    fn handler(
        &self,
        handler: Arc<BoxedHandlerService>,
        req: Request,
        mut middlewares: NextMiddlewaresIterType,
    ) -> BoxFuture<Result<Response, app::ErrorKind>> {
        match middlewares.next() {
            // Call next middleware
            Some(middleware) => middleware.call(handler, req, middlewares),
            // Call handler service
            None => handler.call(req),
        }
    }
}

impl<Func, Fut> Middleware for Func
where
    Func: Fn(Arc<BoxedHandlerService>, Request, NextMiddlewaresIterType) -> Fut
        + Send
        + Sync
        + 'static,
    Fut: Future<Output = Result<Response, app::ErrorKind>> + Send + Sync + 'static,
{
    fn call(
        &self,
        handler: Arc<BoxedHandlerService>,
        req: Request,
        middlewares: NextMiddlewaresIterType,
    ) -> BoxFuture<Result<Response, app::ErrorKind>> {
        Box::pin(self(handler, req, middlewares))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Bot,
        context::Context,
        dispatcher::event::{
            bases::EventReturn, service::ServiceFactory as _, telegram::handler::handler_service,
        },
        types::Update,
    };

    use std::{iter, sync::RwLock};
    use tokio;

    #[tokio::test]
    async fn test_call() {
        let middleware = |handler: Arc<BoxedHandlerService>,
                          req: Request,
                          mut middlewares: NextMiddlewaresIterType| async move {
            match middlewares.next() {
                // Call next middleware
                Some(middleware) => middleware.call(handler, req, middlewares),
                // Call handler service
                None => handler.call(req),
            }
            .await
        };

        let handler_service_factory = handler_service(|| async {}).new_service(());
        let handler_service = Arc::new(handler_service_factory.await.unwrap());

        let req = Request::new(
            Bot::default(),
            Update::default(),
            RwLock::new(Context::default()),
        );

        let res = Middleware::call(&middleware, handler_service, req, Box::new(iter::empty()))
            .await
            .unwrap();
        assert_eq!(*res.response(), EventReturn::default());
    }
}
