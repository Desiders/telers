use crate::{
    dispatcher::{event::EventReturn, router::Request},
    error::AppErrorKind,
};

use async_trait::async_trait;
use std::{future::Future, sync::Arc};

pub type Middlewares = Vec<Arc<Box<dyn Middleware>>>;

#[async_trait]
pub trait Middleware: Send + Sync {
    /// Execute middleware
    /// # Arguments
    /// * `req` - Data for router service
    /// # Returns
    /// Updated [`Request`] for router service and [`EventReturn`] or [`AppErrorKind`]
    /// # Errors
    /// If outer middleware returns error
    #[must_use]
    async fn call(&self, req: Request) -> Result<(Request, EventReturn), AppErrorKind>;
}

#[async_trait]
impl<Func, Fut> Middleware for Func
where
    Func: Fn(Request) -> Fut + Send + Sync,
    Fut: Future<Output = Result<(Request, EventReturn), AppErrorKind>> + Send,
{
    async fn call(&self, request: Request) -> Result<(Request, EventReturn), AppErrorKind> {
        self(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::Bot, context::Context, types::Update};

    use tokio;

    #[tokio::test]
    async fn test_call() {
        let middleware = |request: Request| async move { Ok((request, EventReturn::default())) };

        let request = Request::new(Bot::default(), Update::default(), Context::default());

        let (updated_request, _) = Middleware::call(&middleware, request.clone())
            .await
            .unwrap();
        assert!(request == updated_request);
    }
}
