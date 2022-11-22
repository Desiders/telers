use crate::{
    dispatcher::{
        event::{bases::EventReturn, service::BoxFuture},
        RouterRequest,
    },
    error::app,
};

use std::future::Future;

pub trait Middleware {
    /// Execute middleware
    /// # Arguments
    /// * `req` - Data for router service
    /// # Returns
    /// Updated [`RouterRequest`] for router service and [`EventReturn`] or [`app::Error`].
    /// [`EventReturn`] indicates how the dispatcher should process response, for more information see [`EventReturn`].
    /// # Errors
    /// If outer middleware returns error
    #[must_use]
    fn call(
        &self,
        req: RouterRequest,
    ) -> BoxFuture<Result<(RouterRequest, EventReturn), app::Error>>;
}

impl<Func, Fut> Middleware for Func
where
    Func: Fn(RouterRequest) -> Fut + 'static,
    Fut: Future<Output = Result<(RouterRequest, EventReturn), app::Error>> + 'static,
{
    fn call(
        &self,
        req: RouterRequest,
    ) -> BoxFuture<Result<(RouterRequest, EventReturn), app::Error>> {
        Box::pin(self(req))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::Bot, context::Context, types::Update};

    use std::{cell::RefCell, rc::Rc};

    macro_rules! r#await {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_call() {
        let middleware = |req: RouterRequest| async move { Ok((req, EventReturn::default())) };

        let bot = Rc::new(Bot::default());
        let update = Rc::new(Update::default());
        let context = Rc::new(RefCell::new(Context::default()));
        let req = RouterRequest::new(bot, update, context);

        let (updated_req, _) = r#await!(Middleware::call(&middleware, req.clone())).unwrap();
        assert!(req == updated_req);
    }
}
