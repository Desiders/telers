use crate::{
    errors::EventErrorKind,
    event::{
        service::Service,
        telegram::{BoxedHandlerService, HandlerRequest, HandlerResponse},
    },
};

use async_trait::async_trait;
use std::{future::Future, pin::Pin, sync::Arc};

/// List of middlewares
pub type Middlewares<Client> = Vec<Arc<dyn Middleware<Client>>>;
/// The middleware chain and the handler at the end
pub type Next<Client> = Box<
    dyn Fn(
            HandlerRequest<Client>,
        )
            -> Pin<Box<dyn Future<Output = Result<HandlerResponse<Client>, EventErrorKind>> + Send>>
        + Send
        + Sync,
>;

/// Inner middlewares called after outer middlewares, after filters, but before handlers.
/// If filters aren't passed, then inner middlewares aren't called.
///
/// Prefer to use inner middlewares over outer middlewares in some cases:
/// - If you need to call middlewares after filters and before handlers
/// - If you need to manipulate with call of next middleware or handler
/// - If you need to manipulate with [`HandlerRequest`] or [`HandlerResponse`]
/// Usually inner middlewares are more relevant than outer middlewares.
///
/// Implement this trait for your own middlewares
#[async_trait]
pub trait Middleware<Client>: Send + Sync {
    /// Execute middleware
    /// # Arguments
    /// * `request` - Data for handler and middlewares
    /// * `next` - Call next middleware or handler, if middlewares are empty or already called
    /// # Returns
    /// [`HandlerResponse`] from handler or [`EventErrorKind`] if handler or middleware returns an error
    /// # Errors
    /// If any inner middleware returns an error
    /// If handler returns an error. Probably it's the error to extract args to the handler
    #[must_use]
    async fn call(
        &self,
        request: HandlerRequest<Client>,
        next: Next<Client>,
    ) -> Result<HandlerResponse<Client>, EventErrorKind>;
}

#[async_trait]
impl<T: ?Sized, Client> Middleware<Client> for Arc<T>
where
    T: Middleware<Client>,
    Client: Send + Sync + 'static,
{
    async fn call(
        &self,
        request: HandlerRequest<Client>,
        next: Next<Client>,
    ) -> Result<HandlerResponse<Client>, EventErrorKind> {
        T::call(self, request, next).await
    }
}

/// To possible use function-like as middlewares
#[async_trait]
impl<Client, Func, Fut> Middleware<Client> for Func
where
    Client: Send + Sync + 'static,
    Func: Fn(HandlerRequest<Client>, Next<Client>) -> Fut + Send + Sync,
    Fut: Future<Output = Result<HandlerResponse<Client>, EventErrorKind>> + Send,
{
    async fn call(&self, request: HandlerRequest<Client>, next: Next<Client>) -> Fut::Output {
        self(request, next).await
    }
}

/// Wrap handler and middlewares to [`Next`] function
/// # Notes
/// This function is wrap [`crate::errors::HandlerError`] to [`EventErrorKind::Handler`]
#[must_use]
pub fn wrap_handler_and_middlewares_to_next<T, Client>(
    handler: Arc<BoxedHandlerService<Client>>,
    middlewares: T,
) -> Next<Client>
where
    Client: Send + Sync + 'static,
    T: IntoIterator<Item = Arc<dyn Middleware<Client>>> + Send + Sync + 'static,
    T::IntoIter: Clone + Send + Sync,
{
    let middlewares = middlewares.into_iter();

    Box::new(move |request: HandlerRequest<Client>| {
        let handler = handler.clone();
        let mut middlewares = middlewares.clone();

        Box::pin(async move {
            match middlewares.next() {
                Some(middleware) => {
                    middleware
                        .call(
                            request,
                            wrap_handler_and_middlewares_to_next(handler, middlewares),
                        )
                        .await
                }
                None => match handler.call(request).await {
                    Ok(response) => match response.handler_result {
                        Ok(_) => Ok(response),
                        // If handler returns an error, then wrap it to event error
                        Err(err) => Err(EventErrorKind::Handler(err)),
                    },
                    // If handler service returns an error, then it's an error to extract args to the handler
                    Err(err) => Err(EventErrorKind::Extraction(err)),
                },
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::{Bot, Reqwest},
        context::Context,
        event::{service::ServiceFactory as _, telegram::handler_service, EventReturn},
        types::Update,
    };

    use tokio;

    async fn test_middleware<Client>(
        request: HandlerRequest<Client>,
        next: Next<Client>,
    ) -> Result<HandlerResponse<Client>, EventErrorKind> {
        next(request).await
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
            request,
            wrap_handler_and_middlewares_to_next(handler_service, []),
        )
        .await
        .unwrap();

        match response.handler_result {
            Ok(EventReturn::Finish) => {}
            _ => panic!("Unexpected response"),
        }
    }
}
