use crate::{
    dispatcher::event::{
        service::Service,
        telegram::{BoxedHandlerService, HandlerRequest, HandlerResponse},
    },
    error::AppErrorKind,
};

use async_trait::async_trait;
use std::{future::Future, pin::Pin, sync::Arc};

pub type Middlewares<Client> = Vec<Arc<Box<dyn Middleware<Client>>>>;
pub type Next<Client> = Box<
    dyn Fn(
            HandlerRequest<Client>,
        )
            -> Pin<Box<dyn Future<Output = Result<HandlerResponse<Client>, AppErrorKind>> + Send>>
        + Send
        + Sync,
>;

#[async_trait]
pub trait Middleware<Client>: Send + Sync {
    /// Execute middleware
    /// # Arguments
    /// * `request` - Data for handler and middlewares
    /// * `next` - Call next middleware or handler, if middlewares are empty or already called
    /// # Returns
    /// [`HandlerResponse`] from handler or [`AppErrorKind`] if handler or middleware returns an error
    /// # Errors
    /// If any inner middleware returns an error
    /// If handler returns an error. Probably it's the error to extract args to the handler
    #[must_use]
    async fn call(
        &self,
        request: HandlerRequest<Client>,
        next: Next<Client>,
    ) -> Result<HandlerResponse<Client>, AppErrorKind>;
}

#[async_trait]
impl<Client, Func, Fut> Middleware<Client> for Func
where
    Client: Send + Sync + 'static,
    Func: Fn(HandlerRequest<Client>, Next<Client>) -> Fut + Send + Sync,
    Fut: Future<Output = Result<HandlerResponse<Client>, AppErrorKind>> + Send,
{
    async fn call(&self, request: HandlerRequest<Client>, next: Next<Client>) -> Fut::Output {
        self(request, next).await
    }
}

#[must_use]
pub fn wrap_handler_and_middleware_to_next<Client>(
    handler: Arc<BoxedHandlerService<Client>>,
    middlewares: Middlewares<Client>,
) -> Next<Client>
where
    Client: Send + Sync + 'static,
{
    Box::new(move |request: HandlerRequest<Client>| {
        let handler = handler.clone();
        let middlewares = middlewares.clone();

        Box::pin(async move {
            match middlewares.split_first() {
                Some((middleware, middlewares)) => {
                    let next = Box::new(wrap_handler_and_middleware_to_next(
                        handler,
                        middlewares.to_vec(),
                    ));
                    middleware.call(request, next).await
                }
                None => handler
                    .call(request)
                    .await
                    .map_err(AppErrorKind::Extraction),
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
        dispatcher::event::{service::ServiceFactory as _, telegram::handler_service, EventReturn},
        types::Update,
    };

    use tokio;

    async fn test_middleware<Client>(
        request: HandlerRequest<Client>,
        next: Next<Client>,
    ) -> Result<HandlerResponse<Client>, AppErrorKind> {
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
        let middlewares = vec![];
        let response = Middleware::call(
            &test_middleware,
            request,
            wrap_handler_and_middleware_to_next(handler_service, middlewares),
        )
        .await
        .unwrap();

        match response.handler_result {
            Ok(EventReturn::Finish) => {}
            _ => panic!("Unexpected response"),
        }
    }
}
