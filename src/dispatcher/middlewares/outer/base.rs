use crate::{
    dispatcher::{event::EventReturn, RouterRequest},
    error::AppErrorKind,
};

use async_trait::async_trait;
use std::{future::Future, sync::Arc};

/// List of middlewares
pub type Middlewares<Client> = Vec<Arc<Box<dyn Middleware<Client>>>>;
/// Response from middleware.
/// First element is/isn't updated [`RouterRequest`] and second is [`EventReturn`] for the manipulate processing event,
/// see [`EventReturn`] for more info.
pub type MiddlewareResponse<Client> = (RouterRequest<Client>, EventReturn);

/// Outer middlewares called before filters, inner middlewares and handlers
///
/// Prefer to use outer middlewares over inner middlewares in some cases:
/// - If you need to call middlewares before filters, inner middlewares and handlers
/// - If you need to manipulate with [`RouterRequest`]
/// Usually outer middlewares are used to manipulate with [`RouterRequest`].
///
/// Implement this trait for your own middlewares
#[async_trait]
pub trait Middleware<Client>: Send + Sync {
    /// Execute middleware
    /// # Arguments
    /// * `request` - Data for observers, filters, handler and middlewares
    /// # Errors
    /// If outer middleware returns error
    async fn call(
        &self,
        request: RouterRequest<Client>,
    ) -> Result<MiddlewareResponse<Client>, AppErrorKind>;
}

/// To possible use function-like as middlewares
#[async_trait]
impl<Client, Func, Fut> Middleware<Client> for Func
where
    Client: Send + Sync + 'static,
    Func: Fn(RouterRequest<Client>) -> Fut + Send + Sync,
    Fut: Future<Output = Result<MiddlewareResponse<Client>, AppErrorKind>> + Send,
{
    async fn call(
        &self,
        request: RouterRequest<Client>,
    ) -> Result<MiddlewareResponse<Client>, AppErrorKind> {
        self(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::{Bot, Reqwest},
        context::Context,
        types::Update,
    };

    use tokio;

    #[tokio::test]
    async fn test_call() {
        let middleware =
            |request: RouterRequest<Reqwest>| async move { Ok((request, EventReturn::default())) };

        let request = RouterRequest::new(
            Bot::<Reqwest>::default(),
            Update::default(),
            Context::default(),
        );

        let (updated_request, _) = Middleware::call(&middleware, request.clone())
            .await
            .unwrap();
        assert!(request == updated_request);
    }
}
