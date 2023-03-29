use crate::{
    dispatcher::{event::EventReturn, router::Request},
    error::AppErrorKind,
};

use async_trait::async_trait;
use std::{future::Future, sync::Arc};

pub type Middlewares<Client> = Vec<Arc<Box<dyn Middleware<Client>>>>;
pub type MiddlewareResponse<Client> = (Request<Client>, EventReturn);

#[async_trait]
pub trait Middleware<Client>: Send + Sync {
    /// Execute middleware
    /// # Arguments
    /// * `request` - Data for observers, filters, handler and middlewares
    /// # Returns
    /// Updated [`Request`] and [`EventReturn`] or [`AppErrorKind`]
    /// # Errors
    /// If outer middleware returns error
    async fn call(
        &self,
        request: Request<Client>,
    ) -> Result<MiddlewareResponse<Client>, AppErrorKind>;
}

#[async_trait]
impl<Client, Func, Fut> Middleware<Client> for Func
where
    Client: Send + Sync + 'static,
    Func: Fn(Request<Client>) -> Fut + Send + Sync,
    Fut: Future<Output = Result<MiddlewareResponse<Client>, AppErrorKind>> + Send,
{
    async fn call(
        &self,
        request: Request<Client>,
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
            |request: Request<Reqwest>| async move { Ok((request, EventReturn::default())) };

        let request = Request::new(
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
