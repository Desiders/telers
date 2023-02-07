use crate::{
    dispatcher::event::telegram::{BoxedHandlerService, HandlerRequest, HandlerResponse},
    error::AppErrorKind,
};

use async_trait::async_trait;
use std::{future::Future, sync::Arc};

pub type Middlewares = Vec<Arc<Box<dyn Middleware>>>;
pub type MiddlewaresIter = Box<dyn Iterator<Item = Arc<Box<dyn Middleware>>> + Send>;

#[async_trait]
pub trait Middleware: Send + Sync {
    /// Execute middleware
    /// # Arguments
    /// * `handler` - Handler service
    /// * `request` - Data for handler service
    /// * `middlewares` - Middlewares for handler service
    /// # Returns
    /// [`HandlerResponse`] from handler service or [`AppErrorKind`]
    /// # Errors
    /// If any inner middleware returns error
    /// If handler returns error. Probably it's error to extract args to the handler
    #[must_use]
    async fn call(
        &self,
        handler: Arc<BoxedHandlerService>,
        request: HandlerRequest,
        middlewares: MiddlewaresIter,
    ) -> Result<HandlerResponse, AppErrorKind>;
}

/// Call next middleware or handler service if all middlewares has passed
/// # Arguments
/// * `handler` - Handler service
/// * `request` - Data for handler service
/// * `middlewares` - Middlewares for handler service
/// # Returns
/// [`HandlerResponse`] from handler service or [`AppErrorKind`]
/// # Errors
/// If any inner middleware returns error
/// If handler returns error. Probably it's error to extract args to the handler
pub async fn call_handler(
    handler: Arc<BoxedHandlerService>,
    request: HandlerRequest,
    mut middlewares: MiddlewaresIter,
) -> Result<HandlerResponse, AppErrorKind> {
    match middlewares.next() {
        // Call next middleware
        Some(middleware) => middleware.call(handler, request, middlewares).await,
        // Call handler service
        None => handler
            .call(request)
            .await
            .map_err(AppErrorKind::Extraction),
    }
}

#[async_trait]
impl<Func, Fut> Middleware for Func
where
    Func: Fn(Arc<BoxedHandlerService>, HandlerRequest, MiddlewaresIter) -> Fut + Send + Sync,
    Fut: Future<Output = Result<HandlerResponse, AppErrorKind>> + Send,
{
    async fn call(
        &self,
        handler: Arc<BoxedHandlerService>,
        request: HandlerRequest,
        middlewares: MiddlewaresIter,
    ) -> Fut::Output {
        self(handler, request, middlewares).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Bot,
        context::Context,
        dispatcher::event::{service::ServiceFactory as _, telegram::handler_service, EventReturn},
        types::Update,
    };

    use std::iter;
    use tokio;

    async fn test_middleware(
        handler: Arc<BoxedHandlerService>,
        request: HandlerRequest,
        middlewares: MiddlewaresIter,
    ) -> Result<HandlerResponse, AppErrorKind> {
        call_handler(handler, request, middlewares).await
    }

    #[tokio::test]
    async fn test_call() {
        let handler_service_factory =
            handler_service(|| async { Ok(EventReturn::Finish) }).new_service(());
        let handler_service = Arc::new(handler_service_factory.unwrap());

        let request = HandlerRequest::new(Bot::default(), Update::default(), Context::default());
        let response = Middleware::call(
            &test_middleware,
            handler_service,
            request,
            Box::new(iter::empty()),
        )
        .await
        .unwrap();

        match response.handler_result {
            Ok(EventReturn::Finish) => {}
            _ => panic!("Unexpected response"),
        }
    }
}
