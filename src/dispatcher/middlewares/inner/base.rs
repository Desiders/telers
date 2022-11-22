use crate::{
    dispatcher::event::{
        service::BoxFuture,
        telegram::{BoxedHandlerService, HandlerRequest, HandlerResponse},
    },
    error::app,
};

use std::{future::Future, rc::Rc};

pub trait Middleware {
    /// Execute middleware
    /// # Arguments
    /// * `handler` - Handler service
    /// * `req` - Data for handler service
    /// * `middlewares` - Middlewares for handler service
    /// # Returns
    /// [`HandlerResponse`] from handler service or [`app::Error`]
    /// # Errors
    /// If any inner middleware returns error
    /// If handler returns error. Probably it's error to extract args to the handler
    #[must_use]
    fn call(
        &self,
        handler: Rc<BoxedHandlerService>,
        req: HandlerRequest,
        middlewares: Box<dyn Iterator<Item = Rc<Box<dyn Middleware>>>>,
    ) -> BoxFuture<Result<HandlerResponse, app::Error>>;

    /// Call next middleware or handler service if all middlewares has passed
    /// # Arguments
    /// * `handler` - Handler service
    /// * `req` - Data for handler service
    /// * `middlewares` - Middlewares for handler service
    /// # Returns
    /// [`HandlerResponse`] from handler service or [`app::Error`]
    /// # Errors
    /// If any inner middleware returns error
    /// If handler returns error. Probably it's error to extract args to the handler
    #[must_use]
    fn handler(
        &self,
        handler: Rc<BoxedHandlerService>,
        req: HandlerRequest,
        mut middlewares: Box<dyn Iterator<Item = Rc<Box<dyn Middleware>>>>,
    ) -> BoxFuture<Result<HandlerResponse, app::Error>> {
        match middlewares.next() {
            // Call next middleware
            Some(middleware) => middleware.call(handler, req, middlewares),
            // Call handler service
            None => handler.call(req),
        }
    }
}

impl<Func, Fut> Middleware for Func
where
    Func: Fn(
            Rc<BoxedHandlerService>,
            HandlerRequest,
            Box<dyn Iterator<Item = Rc<Box<dyn Middleware>>>>,
        ) -> Fut
        + 'static,
    Fut: Future<Output = Result<HandlerResponse, app::Error>> + 'static,
{
    fn call(
        &self,
        handler: Rc<BoxedHandlerService>,
        req: HandlerRequest,
        middlewares: Box<dyn Iterator<Item = Rc<Box<dyn Middleware>>>>,
    ) -> BoxFuture<Result<HandlerResponse, app::Error>> {
        Box::pin(self(handler, req, middlewares))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Bot,
        context::Context,
        dispatcher::event::{
            bases::EventReturn, service::ServiceFactory as _, telegram::handler_service,
        },
        types::Update,
    };

    use std::{cell::RefCell, iter};

    macro_rules! r#await {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_call() {
        let middleware =
            |handler: Rc<BoxedHandlerService>,
             req: HandlerRequest,
             mut middlewares: Box<dyn Iterator<Item = Rc<Box<dyn Middleware>>>>| async move {
                match middlewares.next() {
                    // Call next middleware
                    Some(middleware) => middleware.call(handler, req, middlewares),
                    // Call handler service
                    None => handler.call(req),
                }
                .await
            };

        let handler_service_factory = handler_service(|| async {}).new_service(());
        let handler_service = Rc::new(r#await!(handler_service_factory).unwrap());

        let bot = Rc::new(Bot::default());
        let update = Rc::new(Update::default());
        let context = Rc::new(RefCell::new(Context::default()));
        let req = HandlerRequest::new(bot, update, context);

        let res = r#await!(Middleware::call(
            &middleware,
            handler_service,
            req,
            Box::new(iter::empty())
        ))
        .unwrap();
        assert_eq!(*res.response(), EventReturn::default());
    }
}
