use crate::{
    dispatcher::{
        event::{bases::EventReturn, service::BoxFuture},
        RouterRequest,
    },
    error::app,
};

use std::{future::Future, sync::Arc};

pub type MiddlewareType = Box<dyn Middleware + Send + Sync>;
pub type Middlewares = Vec<Arc<MiddlewareType>>;

pub trait Middleware: Send + Sync {
    /// Execute middleware
    /// # Arguments
    /// * `req` - Data for router service
    /// # Returns
    /// Updated [`RouterRequest`] for router service and [`EventReturn`] or [`app::ErrorKind`].
    /// [`EventReturn`] indicates how the dispatcher should process response, for more information see [`EventReturn`].
    /// # Errors
    /// If outer middleware returns error
    #[must_use]
    fn call(
        &self,
        req: RouterRequest,
    ) -> BoxFuture<Result<(RouterRequest, EventReturn), app::ErrorKind>>;
}

impl<Func, Fut> Middleware for Func
where
    Func: Fn(RouterRequest) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(RouterRequest, EventReturn), app::ErrorKind>>
        + Send
        + Sync
        + 'static,
{
    fn call(
        &self,
        req: RouterRequest,
    ) -> BoxFuture<Result<(RouterRequest, EventReturn), app::ErrorKind>> {
        Box::pin(self(req))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::Bot, context::Context, types::Update};

    use std::sync::RwLock;
    use tokio;

    #[tokio::test]
    async fn test_call() {
        let middleware = |req: RouterRequest| async move { Ok((req, EventReturn::default())) };

        let req = RouterRequest::new(
            Bot::default(),
            Update::default(),
            RwLock::new(Context::default()),
        );

        let (updated_req, _) = Middleware::call(&middleware, req.clone()).await.unwrap();
        assert!(req == updated_req);
    }
}
