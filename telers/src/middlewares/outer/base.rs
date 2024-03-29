use crate::{client::Reqwest, errors::EventErrorKind, event::EventReturn, router::Request};

use async_trait::async_trait;
use std::{future::Future, sync::Arc};

/// Response from middleware.
/// First element is/isn't updated [`Request`] and second is [`EventReturn`] for the manipulate processing event,
/// see [`EventReturn`] for more info.
pub type MiddlewareResponse<Client = Reqwest> = (Request<Client>, EventReturn);

/// Outer middlewares called before filters, inner middlewares and handlers
///
/// Prefer to use outer middlewares over inner middlewares in some cases:
/// - If you need to call middlewares before filters, inner middlewares and handlers
/// - If you need to manipulate with [`Request`] and [`crate::context::Context`] in it
/// Usually outer middlewares are used to manipulate with [`Request`].
///
/// Implement this trait for your own middlewares
#[async_trait]
pub trait Middleware<Client = Reqwest>: Send + Sync {
    /// Execute middleware
    /// # Arguments
    /// * `request` - Data for observers, filters, handler and middlewares
    /// # Errors
    /// If outer middleware returns error
    async fn call(
        &self,
        request: Request<Client>,
    ) -> Result<MiddlewareResponse<Client>, EventErrorKind>;
}

#[async_trait]
impl<T: ?Sized, Client> Middleware<Client> for Arc<T>
where
    T: Middleware<Client>,
    Client: Send + Sync + 'static,
{
    async fn call(
        &self,
        request: Request<Client>,
    ) -> Result<MiddlewareResponse<Client>, EventErrorKind> {
        T::call(self, request).await
    }
}

/// To possible use function-like as middlewares
#[async_trait]
impl<Client, Func, Fut> Middleware<Client> for Func
where
    Client: Send + Sync + 'static,
    Func: Fn(Request<Client>) -> Fut + Send + Sync,
    Fut: Future<Output = Result<MiddlewareResponse<Client>, EventErrorKind>> + Send,
{
    async fn call(
        &self,
        request: Request<Client>,
    ) -> Result<MiddlewareResponse<Client>, EventErrorKind> {
        self(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::{Bot, Reqwest},
        context::Context,
        types::{Message, Update, UpdateKind},
    };

    use tokio;

    #[tokio::test]
    async fn test_call() {
        let middleware =
            |request: Request<Reqwest>| async move { Ok((request, EventReturn::default())) };

        let request = Request::new(
            Arc::new(Bot::<Reqwest>::default()),
            Arc::new(Update {
                id: 0,
                kind: UpdateKind::Message(Message::default()),
            }),
            Arc::new(Context::default()),
        );

        let (updated_request, _) = Middleware::call(&middleware, request.clone())
            .await
            .unwrap();
        assert!(request == updated_request);
    }
}
