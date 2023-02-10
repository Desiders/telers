use crate::{
    dispatcher::event::telegram::{BoxedHandlerService, HandlerRequest, HandlerResponse},
    error::AppErrorKind,
};

use async_trait::async_trait;
use std::{future::Future, sync::Arc};

pub type Middlewares<Client> = Vec<Arc<Box<dyn Middleware<Client>>>>;
pub type MiddlewaresIter<Client> =
    Box<dyn Iterator<Item = Arc<Box<dyn Middleware<Client>>>> + Send>;

#[async_trait]
pub trait Middleware<Client>: Send + Sync {
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
        handler: Arc<BoxedHandlerService<Client>>,
        request: HandlerRequest<Client>,
        middlewares: MiddlewaresIter<Client>,
    ) -> Result<HandlerResponse<Client>, AppErrorKind>;
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
pub async fn call_handler<Client>(
    handler: Arc<BoxedHandlerService<Client>>,
    request: HandlerRequest<Client>,
    mut middlewares: MiddlewaresIter<Client>,
) -> Result<HandlerResponse<Client>, AppErrorKind> {
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
impl<Client, Func, Fut> Middleware<Client> for Func
where
    Client: Send + Sync + 'static,
    Func: Fn(Arc<BoxedHandlerService<Client>>, HandlerRequest<Client>, MiddlewaresIter<Client>) -> Fut
        + Send
        + Sync,
    Fut: Future<Output = Result<HandlerResponse<Client>, AppErrorKind>> + Send,
{
    async fn call(
        &self,
        handler: Arc<BoxedHandlerService<Client>>,
        request: HandlerRequest<Client>,
        middlewares: MiddlewaresIter<Client>,
    ) -> Fut::Output {
        self(handler, request, middlewares).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::{Bot, Reqwest},
        context::Context,
        dispatcher::event::{service::ServiceFactory as _, telegram::handler_service, EventReturn},
        types::Update,
    };

    use std::iter;
    use tokio;

    async fn test_middleware<Client>(
        handler: Arc<BoxedHandlerService<Client>>,
        request: HandlerRequest<Client>,
        middlewares: MiddlewaresIter<Client>,
    ) -> Result<HandlerResponse<Client>, AppErrorKind> {
        call_handler(handler, request, middlewares).await
    }

    #[tokio::test]
    async fn test_call() {
        let handler_service_factory =
            handler_service(|| async { Ok(EventReturn::Finish) }).new_service(());
        let handler_service = Arc::new(handler_service_factory.unwrap());

        let request = HandlerRequest::new(
            Bot::<Reqwest>::default(),
            Update::default(),
            Context::default(),
        );
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
